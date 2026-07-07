#!/bin/bash
# after current ref2 finishes, alternate refine/detail forever (watchdog submits)
dd=$1
until grep -q "^final" logs_t0${dd}_ref2.txt 2>/dev/null; do sleep 60; done
for i in 1 2 3; do
  python3 -u solve_detail.py $dd 70 > logs_t0${dd}_det$i.txt 2>&1
  python3 -u refine.py $dd 5 > logs_t0${dd}_ref$((i+2)).txt 2>&1
done
echo loop$dd done
