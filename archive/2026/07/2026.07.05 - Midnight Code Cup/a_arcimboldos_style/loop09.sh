#!/bin/bash
# t09: alternate wide refine and detail forever
until ! pgrep -f wr9.py >/dev/null; do sleep 120; done
cp out/09w.out out/09.out 2>/dev/null
i=0
while true; do
  i=$((i+1))
  python3 -u solve_detail.py 9 300 20 0 > logs_t9_fdet$((i%3)).txt 2>&1
  cp out/09.out out/09w.out; python3 -u wr9.py > logs_t9_fwr$((i%3)).txt 2>&1
  cp out/09w.out out/09.out 2>/dev/null
done
