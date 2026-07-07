#!/bin/bash
# every 10 min: promote 03s.out to 03.out if better (watchdog then submits)
while true; do
  sleep 600
  python3 - <<'PY'
import sys, os, shutil
sys.path.insert(0,'.')
import mcc_a
try:
    s = mcc_a.score_file(3, 'out/03s.out')
    m = mcc_a.score_file(3, 'out/03.out')
    if s < m:
        shutil.copy('out/03s.out', 'out/03.out')
        print(f'promoted {s:,}', flush=True)
except Exception as e:
    print(e)
PY
done
