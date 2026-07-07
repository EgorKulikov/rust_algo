"""HODL trading daemon: poll prices ~1 Hz, per-stock direction model, bang-bang position.

Run via ./run_daemon.sh (supervisor). Logs: work/daemon.log, work/prices.jsonl,
work/trades.jsonl, work/snap.jsonl. strategies.py is hot-reloaded every loop.
Trades execute on a thread pool so bursts never stall the poll loop.
"""

import importlib
import json
import os
import sys
import threading
import time
import traceback
from concurrent.futures import ThreadPoolExecutor

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hodl
import strategies

WORK = os.path.join(os.path.dirname(os.path.abspath(__file__)), "work")
os.makedirs(WORK, exist_ok=True)

SNAP_EVERY = 60.0
HIST_EVERY = 900.0
HIST_CAP = 6000

_loglock = threading.Lock()


def log(msg):
    line = time.strftime("%Y-%m-%d %H:%M:%S") + " " + msg
    with _loglock:
        print(line, flush=True)
        with open(os.path.join(WORK, "daemon.log"), "a") as f:
            f.write(line + "\n")


def jlog(name, obj):
    with _loglock:
        with open(os.path.join(WORK, name), "a") as f:
            f.write(json.dumps(obj, separators=(",", ":")) + "\n")


class Model:
    __slots__ = ("price", "s_m1", "s_lnz", "stats", "hist")

    def __init__(self):
        self.price = None
        self.s_m1 = 0
        self.s_lnz = 0
        self.stats = {}
        self.hist = []

    def bump(self, key, dp):
        cnt, sm = self.stats.get(key, (0, 0.0))
        self.stats[key] = (cnt + 1, sm + dp)

    def observe(self, t, price, learn=True):
        if self.price is None:
            self.price = price
            self.hist.append((t, price))
            return
        dp = price - self.price
        if dp == 0:
            return
        if learn:
            self.bump(("m1", self.s_m1), dp)
            self.bump(("lnz", self.s_lnz), dp)
        sign = 1 if dp > 0 else -1
        self.s_m1 = sign
        self.s_lnz = sign
        self.price = price
        self.hist.append((t, price))
        if len(self.hist) > HIST_CAP:
            del self.hist[: len(self.hist) - HIST_CAP]


def build_models():
    models = {}
    data = hodl.full_history()
    for s in data["stocks"]:
        m = Model()
        for e in s["history"]:
            m.observe(e["time"], e["price"])
        models[s["stock_id"]] = m
    return models, data["current_time"]


def main():
    log("daemon starting (parallel trades)")
    models, ct = build_models()
    log(f"models warmed from history, current_time={ct}")
    st = hodl.state()
    pos = {p["stock_id"]: p["amount"] for p in st["positions"]}
    for s in st["stocks"]:
        pos.setdefault(s["stock_id"], 0)

    poslock = threading.Lock()
    inflight = set()
    need_resync = threading.Event()
    pool = ThreadPoolExecutor(max_workers=6)

    def trade_worker(sid, want):
        try:
            with poslock:
                have = pos.get(sid, 0)
            if want == have:
                return
            action = "buy" if want > have else "sell"
            r = hodl.trade(action, sid, abs(want - have))
            with poslock:
                pos[sid] = r["result"]["position"]["amount"]
            jlog("trades.jsonl", r["result"]["trade"])
        except hodl.ApiError as e:
            log(f"trade {sid} -> {want} failed: {e}")
            need_resync.set()
        except Exception:
            log(f"trade {sid} error:\n" + traceback.format_exc())
            need_resync.set()
        finally:
            inflight.discard(sid)

    last_snap = 0.0
    last_hist = time.monotonic()
    last_stand = 0.0

    while True:
        loop_start = time.monotonic()
        try:
            strategies_mod = importlib.reload(strategies)
        except Exception:
            log("strategies reload FAILED, keeping old:\n" + traceback.format_exc())
            strategies_mod = strategies

        try:
            pr = hodl.prices()
            t = pr["current_time"]
            if t >= 86400000 - 2000:
                log("contest over; idling")
                time.sleep(30)
                continue
            for e in pr["prices"]:
                sid, p = e["stock_id"], e["current_price"]
                m = models.get(sid)
                if m is None:
                    m = models[sid] = Model()
                if m.price != p:
                    m.observe(t, p)
                    jlog("prices.jsonl", {"t": t, "sid": sid, "p": p})

            # evaluate EVERY stock every loop; dispatch trades asynchronously
            for sid, m in models.items():
                if m.price is None or sid in inflight:
                    continue
                try:
                    want = int(strategies_mod.desired(sid, m, t=t, models=models))
                except Exception:
                    log(f"desired({sid}) FAILED:\n" + traceback.format_exc())
                    continue
                want = max(0, min(100, want))
                with poslock:
                    have = pos.get(sid, 0)
                if want != have:
                    inflight.add(sid)
                    pool.submit(trade_worker, sid, want)

            now = time.monotonic()
            if need_resync.is_set() or now - last_snap > SNAP_EVERY:
                last_snap = now
                need_resync.clear()
                st = hodl.state()
                with poslock:
                    for p_ in st["positions"]:
                        if p_["stock_id"] not in inflight:
                            pos[p_["stock_id"]] = p_["amount"]
                price_now = {s["stock_id"]: s["current_price"] for s in st["stocks"]}
                profits = {}
                for r in st["realized"]:
                    sid = r["stock_id"]
                    profits[sid] = r["realized"] + pos.get(sid, 0) * price_now.get(sid, 0)
                jlog("snap.jsonl", {"t": st["current_time"], "profits": profits, "pos": pos})
                total = sum(max(0, v) for v in profits.values())
                log(f"snap t={st['current_time']//60000}min positive_profit_total={total}")

            if now - last_stand > 600.0:
                last_stand = now
                try:
                    import urllib.request as _ur
                    req = _ur.Request("https://finals.midnightcodecup.org/api/v1/standings/?problem=H")
                    req.add_header("Authorization", "Bearer " + hodl.TOKEN)
                    with _ur.urlopen(req, timeout=15) as resp:
                        sd = json.loads(resp.read())
                    us = next((tm for tm in sd.get("teams", []) if tm["team"]["id"] == 15), None)
                    jlog("standings.jsonl", {"ts": time.time(),
                        "best": {c["title"]: c["best"][0] for c in sd["header"] if c.get("best")},
                        "us": us and {"rank": us["rank"], "total": us["total"],
                                      "cells": [c["meta"][0] if c["meta"] else None for c in us["cells"]]}})
                    if us:
                        log(f"standings: rank {us['rank']} total {us['total']:.1f}")
                except Exception as e:
                    log(f"standings fetch failed: {e}")

            if now - last_hist > HIST_EVERY:
                last_hist = now
                models, _ = build_models()
                log("models rebuilt from full history")
        except Exception:
            log("loop error:\n" + traceback.format_exc())
            time.sleep(2)

        elapsed = time.monotonic() - loop_start
        time.sleep(max(0.05, 1.0 - elapsed))


if __name__ == "__main__":
    main()
