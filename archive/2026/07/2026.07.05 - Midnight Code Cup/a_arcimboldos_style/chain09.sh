#!/bin/bash
until grep -q "^done:" logs_t09_det.txt 2>/dev/null; do sleep 60; done
for i in 1 2; do
  python3 -u refine.py 9 4 > logs_t09_ref$i.txt 2>&1
  python3 -u solve_detail.py 9 400 20 8 > logs_t09_det$i.txt 2>&1
done
echo chain09 done
