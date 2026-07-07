#!/bin/bash
# endless improve loop alternating detail, standard refine, wide refine
dd=$1; maxdim=$2
i=0
while true; do
  i=$((i+1))
  python3 -u solve_detail.py $dd $maxdim 30 8 > logs_t${dd}_fdet$((i%2)).txt 2>&1
  if [ $((i%2)) -eq 0 ]; then
    python3 -u refine.py $dd 5 > logs_t${dd}_fref$((i%2)).txt 2>&1
  else
    cp out/0${dd}.out out/0${dd}w.out 2>/dev/null || cp out/${dd}.out out/${dd}w.out
    python3 -u wr456.py $dd out/0${dd}w.out > logs_t${dd}_fwr.txt 2>&1 || \
    python3 -u wr456.py $dd out/${dd}w.out > logs_t${dd}_fwr.txt 2>&1
  fi
done
