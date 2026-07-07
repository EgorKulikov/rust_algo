#!/bin/bash
# generic improve loop: detail + refine cycles with fixed refine
dd=$1; maxdim=$2
for i in 4 5 6 7; do
  python3 -u solve_detail.py $dd $maxdim 30 8 > logs_t0${dd}_det$i.txt 2>&1
  python3 -u refine.py $dd 5 > logs_t0${dd}_ref$i.txt 2>&1
done
echo loop_gen $dd done
