#!/usr/bin/env python3
"""Round daemon for MCC problem I: three threads, one per game.

Each round: log last-round feedback to work/<game>.jsonl, hot-reload
strategies.py, compute and submit a move. A failure in one iteration or one
game never kills the rest. Restart-safe: history is rebuilt from the jsonl.

Run:  python3 daemon.py   (from tasks/i; logs to work/daemon.log)
"""
import importlib
import json
import threading
import time
import traceback
from datetime import datetime, timezone
from pathlib import Path

import mcc
import strategies

TASK_DIR = Path(__file__).resolve().parent
WORK = TASK_DIR / "work"
WORK.mkdir(exist_ok=True)

_log_lock = threading.Lock()


def log(game, msg):
    line = f"[{datetime.now(timezone.utc).strftime('%H:%M:%S')}] [{game}] {msg}"
    with _log_lock:
        print(line, flush=True)
        with open(WORK / "daemon.log", "a") as f:
            f.write(line + "\n")


def jsonl_append(game, entry):
    with open(WORK / f"{game}.jsonl", "a") as f:
        f.write(json.dumps(entry) + "\n")


def load_history(game):
    path = WORK / f"{game}.jsonl"
    hist = []
    if path.exists():
        for line in path.read_text().splitlines():
            e = json.loads(line)
            if e.get("type") == "feedback":
                hist.append(e["data"])
    return hist


def reload_strategies(game):
    try:
        importlib.reload(strategies)
    except Exception:
        log(game, "strategies reload FAILED, using previous version:\n" + traceback.format_exc())
    return strategies


def fetch_feedback(game, history, seen):
    """Pull /last-round; append to history+jsonl if it's a new round."""
    try:
        fb = mcc.last_round(game)
    except mcc.ApiError as e:
        if e.code == 404:
            return
        raise
    if fb and fb.get("round") and fb["round"] not in seen:
        seen.add(fb["round"])
        history.append(fb)
        jsonl_append(game, {"type": "feedback", "data": fb})
        mine = fb.get("your_move")
        log(game, f"feedback r{fb['round']}: {json.dumps(mine)}")


def wait_next_round(game, last_round=None, extra=1.0):
    """Sleep to the round boundary, then short-poll until the server actually
    rolls the round. A single post-sleep status can land a hair early and
    still show the old round — the naive version then re-waited a FULL round,
    silently skipping submissions (cost ~15-30 rounds/game by r722)."""
    st = mcc.status(game)
    if last_round is None:
        last_round = st["current_round"]
    time.sleep(max(1.0, st["seconds_to_next_round"] + extra))
    for _ in range(14):
        st = mcc.status(game)
        if st["current_round"] != last_round:
            return st
        time.sleep(2.5)
    return st


def simple_loop(game, make_move):
    """unique-smallest and blotto: one submission per round."""
    history = load_history(game)
    seen = {fb["round"] for fb in history}
    cfg = mcc.config(game)
    last_played = None
    st = mcc.status(game)
    while True:
        try:
            rnd = st["current_round"]
            if rnd == 0:
                log(game, "not started yet")
                time.sleep(20)
                st = mcc.status(game)
                continue
            if rnd != last_played:
                fetch_feedback(game, history, seen)
                strat = reload_strategies(game)
                ctx = {"round": rnd, "config": cfg, "history": history}
                move = make_move(strat, ctx)
                log(game, f"r{rnd}: submitted {move}")
                last_played = rnd
            st = wait_next_round(game, last_round=rnd)
        except Exception:
            log(game, "iteration error:\n" + traceback.format_exc())
            time.sleep(5)
            try:
                st = mcc.status(game)
            except Exception:
                time.sleep(10)


def us_move(strat, ctx):
    n = strat.unique_smallest(ctx)
    mcc.submit_number(n)
    return n


def blotto_move(strat, ctx):
    alloc = strat.blotto(ctx)
    mcc.submit_allocation(alloc)
    return alloc


def door_loop():
    game = mcc.DOOR
    history = load_history(game)
    seen = {fb["round"] for fb in history}
    cfg = mcc.config(game)
    last_played = None
    st = mcc.status(game)
    while True:
        try:
            rnd = st["current_round"]
            gs = st.get("game_state") or {}
            phase = gs.get("phase")
            if rnd == 0 or phase == "not_started":
                log(game, "not started yet")
                time.sleep(20)
                st = mcc.status(game)
                continue
            if phase != "phase_1" or rnd == last_played:
                # joined mid-round or already played: wait for the NEXT round
                st = wait_next_round(game, last_round=rnd)
                continue

            fetch_feedback(game, history, seen)
            strat = reload_strategies(game)
            ctx = {
                "round": rnd,
                "config": cfg,
                "history": history,
                "rooms": gs["rooms"],
                "rush_bonus": gs["rush_bonus"],
            }
            entrance, mode = strat.door_phase1(ctx)
            mcc.submit_entrance(entrance, mode)
            last_played = rnd
            log(game, f"r{rnd} p1: entrance={entrance} mode={mode}")
            jsonl_append(game, {"type": "move", "round": rnd,
                                "entrance": entrance, "mode": mode})

            if mode == "flex":
                # sleep into phase 2, then read counts and commit a room
                st = mcc.status(game)
                gs = st.get("game_state") or {}
                if gs.get("phase") == "phase_1":
                    time.sleep(max(1.0, gs["seconds_to_next_phase"] + 1.5))
                counts = mcc.entrance_counts()
                strat = reload_strategies(game)
                ctx.update(counts=counts, entrance=entrance)
                room = strat.door_phase2(ctx)
                mcc.submit_room(room)
                log(game, f"r{rnd} p2: counts={counts} -> room={room}")
                jsonl_append(game, {"type": "move2", "round": rnd,
                                    "counts": counts, "room": room})
            st = wait_next_round(game, last_round=rnd)
        except Exception:
            log(game, "iteration error:\n" + traceback.format_exc())
            time.sleep(5)
            try:
                st = mcc.status(game)
            except Exception:
                time.sleep(10)


def standings_loop():
    import urllib.request
    url = "https://finals.midnightcodecup.org/api/v1/standings/?problem=I"
    while True:
        try:
            req = urllib.request.Request(
                url, headers={"Authorization": f"Bearer {mcc.KEY}"})
            with urllib.request.urlopen(req, timeout=20) as resp:
                data = json.loads(resp.read())
            with open(WORK / "standings.jsonl", "a") as f:
                f.write(json.dumps({"ts": time.time(), "data": data}) + "\n")
        except Exception:
            log("standings", "poll error:\n" + traceback.format_exc())
        time.sleep(300)


def main():
    threads = [
        threading.Thread(target=simple_loop, args=(mcc.US, us_move), daemon=True),
        threading.Thread(target=simple_loop, args=(mcc.BLOTTO, blotto_move), daemon=True),
        threading.Thread(target=door_loop, daemon=True),
        threading.Thread(target=standings_loop, daemon=True),
    ]
    log("main", "daemon starting")
    for t in threads:
        t.start()
    while True:
        time.sleep(60)


if __name__ == "__main__":
    main()
