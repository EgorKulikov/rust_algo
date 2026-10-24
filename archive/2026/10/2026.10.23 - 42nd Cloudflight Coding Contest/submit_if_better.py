#!/usr/bin/env python3
"""submit_if_better.py <slug> <level> <fileId> <path>: submit only if the file has fewer lines than the server's best for that fileId."""
import json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ccc
slug, level, fid, path = sys.argv[1], int(sys.argv[2]), sys.argv[3], sys.argv[4]
g = ccc.Game(slug)
state = g.info()["score"]["state"].get(f"level{level}", {})
best = min((abs(s["optimizationScore"]) for s in state.get("submissions", []) if s.get("fileId") == fid and s.get("result", {}).get("correct")), default=None)
mine = sum(1 for l in open(path) if l.strip())
if best is not None and mine >= best:
    print(f"{fid}: {mine} not better than server best {best}, skipped")
    sys.exit(0)
code, res = g.submit(level, fid, path)
ev = res.get("evaluation", {}) if isinstance(res, dict) else {}
print(f"{fid}: {mine} vs server best {best} -> http {code} correct {ev.get('isCorrect')} score {ev.get('optimizationScore')}")
