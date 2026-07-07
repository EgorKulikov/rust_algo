#!/usr/bin/env python3
"""Sync all our submission scores for problem B into scores.json."""
import json
import time
import urllib.request
from pathlib import Path

TASK_DIR = Path(__file__).resolve().parent
API = "https://finals.midnightcodecup.org/api/v1"
KEY = (TASK_DIR / "../api-key.txt").read_text().strip()
OUT = TASK_DIR / "scores.json"


def api_get(path: str):
    req = urllib.request.Request(API + path, headers={"Authorization": f"Bearer {KEY}"})
    with urllib.request.urlopen(req, timeout=20) as resp:
        return json.loads(resp.read())


def main() -> None:
    old = json.loads(OUT.read_text()) if OUT.exists() else {}
    subs = api_get("/problems/B/submissions")
    for s in subs:
        known = [r for r in old.values() if r.get("submission") == s["id"]]
        if known and known[0].get("status") == "JUDGED":
            continue
        d = api_get(f"/submissions/{s['id']}")
        for t in d.get("tests", []):
            points = None
            log = t.get("log") or ""
            if log.startswith("points "):
                points = float(log.split()[1])
            old[t["test"]] = {
                "submission": s["id"],
                "status": d.get("status"),
                "points": points,
            }
        time.sleep(1)
    OUT.write_text(json.dumps(dict(sorted(old.items())), indent=1) + "\n")
    scored = {k: v["points"] for k, v in old.items() if v.get("points") is not None}
    total = 0.0
    for stage in sorted({k.split("-")[0] for k in scored}):
        pts = sum(2 ** (10 - int(r.split("-")[1])) * p for r, p in scored.items() if r.startswith(stage))
        total += pts
        print(f"stage {stage}: {pts:.2f}/1023")
    print(f"records scored: {len(scored)}, total stage points: {total:.2f}")


if __name__ == "__main__":
    main()
