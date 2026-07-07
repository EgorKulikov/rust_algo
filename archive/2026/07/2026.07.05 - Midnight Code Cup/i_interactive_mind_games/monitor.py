#!/usr/bin/env python3
"""Watchdog for the task-I daemon. Prints one line per event; a wrapper
(Claude Monitor) turns lines into notifications.

Alerts (immediate): daemon down, new iteration errors, per-game performance
below floor. Heartbeat (every 30 min): per-game recent averages + standings
pace. Checks every 60 s.
"""
import json
import subprocess
import time
from pathlib import Path

TASK_DIR = Path(__file__).resolve().parent
WORK = TASK_DIR / "work"

FLOORS = {"unique-smallest": 0.05, "blotto": 0.08, "door-rush": 80.0}
SCORE_KEY = {"unique-smallest": "rank_score", "blotto": "rank_score",
             "door-rush": "reward_score"}
US_TEAM = "Dvoe protiv vetra"


def daemon_alive():
    try:
        out = subprocess.run(["pgrep", "-f", "daemon.py"], capture_output=True,
                             text=True).stdout.split()
        for pid in out:
            try:
                cwd = Path(f"/proc/{pid}/cwd").resolve()
                cmd = Path(f"/proc/{pid}/cmdline").read_bytes().split(b"\0")[0]
                if cwd == TASK_DIR and b"python" in cmd:
                    return True
            except OSError:
                continue
    except Exception:
        pass
    return False


def recent_scores(game, n=15):
    path = WORK / f"{game}.jsonl"
    if not path.exists():
        return []
    rows = []
    for line in path.read_text().splitlines():
        e = json.loads(line)
        if e.get("type") == "feedback" and (e["data"].get("your_move") or {}):
            rows.append(e["data"])
    key = SCORE_KEY[game]
    return [(fb["round"], fb["your_move"].get(key, 0.0)) for fb in rows[-n:]]


def pace_line():
    path = WORK / "standings.jsonl"
    if not path.exists():
        return ""
    snaps = [json.loads(l) for l in path.read_text().splitlines()]
    # the checksystem occasionally returns transient partial rows — skip
    # snapshots where most teams lack cells
    def full(s):
        return sum(1 for t in s["data"]["teams"]
                   if t.get("cells") and all(c and c.get("meta")
                                             for c in t["cells"]))
    snaps = [s for s in snaps if full(s) >= 20]
    if len(snaps) < 2:
        return ""
    new = snaps[-1]
    old = None
    for s in reversed(snaps[:-1]):
        if new["ts"] - s["ts"] >= 45 * 60:
            old = s
            break
    if old is None:
        old = snaps[0]
    if new["ts"] - old["ts"] < 10 * 60:
        return ""

    def rawmap(snap, gi):
        out = {}
        for t in snap["data"]["teams"]:
            cells = t.get("cells") or []
            if gi < len(cells) and cells[gi] and cells[gi].get("meta"):
                out[t["team"]["name"]] = float(cells[gi]["meta"][0])
        return out

    parts = []
    for gi, g in enumerate(["unique-smallest", "blotto", "door-rush"]):
        ra, rb = rawmap(old, gi), rawmap(new, gi)
        deltas = sorted(((rb[n2] - ra[n2], n2) for n2 in rb if n2 in ra),
                        reverse=True)
        pos = next((i + 1 for i, (_, n2) in enumerate(deltas) if n2 == US_TEAM),
                   None)
        parts.append(f"{g.split('-')[0]}:pace#{pos}/{len(deltas)}")
    mins = (new["ts"] - old["ts"]) / 60
    return f" | pace over {mins:.0f}m " + " ".join(parts)


def main():
    seen_errors = 0
    last_heartbeat = 0.0
    alerted_floor = set()
    alerted_dead = False
    while True:
        try:
            log = (WORK / "daemon.log").read_text().splitlines()
            errs = [l for l in log if "iteration error" in l
                    or "reload FAILED" in l]
            if len(errs) > seen_errors:
                print(f"ALERT: {len(errs) - seen_errors} new daemon errors; "
                      f"last: {errs[-1][:160]}", flush=True)
                seen_errors = len(errs)

            if not daemon_alive():
                if not alerted_dead:
                    print("ALERT: tasks/i daemon is DOWN", flush=True)
                    alerted_dead = True
            else:
                if alerted_dead:
                    print("daemon back up", flush=True)
                alerted_dead = False

            stats = {}
            for g, floor in FLOORS.items():
                sc = recent_scores(g)
                if len(sc) < 10:
                    continue
                mean = sum(v for _, v in sc) / len(sc)
                stats[g] = (mean, sc[-1][0])
                if mean < floor and g not in alerted_floor:
                    print(f"ALERT: {g} degraded: last-{len(sc)} avg "
                          f"{mean:.3f} < floor {floor} (round {sc[-1][0]})",
                          flush=True)
                    alerted_floor.add(g)
                elif mean >= floor * 1.3 and g in alerted_floor:
                    print(f"{g} recovered: avg {mean:.3f}", flush=True)
                    alerted_floor.discard(g)

            now = time.time()
            if now - last_heartbeat >= 30 * 60 and stats:
                last_heartbeat = now
                desc = "  ".join(
                    f"{g}: {m:.3f}@r{r}" for g, (m, r) in stats.items())
                print(f"status: {desc}{pace_line()}", flush=True)
        except Exception as e:
            print(f"monitor error: {e!r}", flush=True)
            time.sleep(120)
        time.sleep(60)


if __name__ == "__main__":
    main()
