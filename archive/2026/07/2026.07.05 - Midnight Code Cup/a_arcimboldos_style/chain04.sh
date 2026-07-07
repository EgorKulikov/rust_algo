#!/bin/bash
until grep -q "^final" logs_t04.txt 2>/dev/null; do sleep 60; done
python3 mcc_a.py submit 4
for i in 1 2; do
  python3 -u refine.py 4 6 > logs_t04_ref$i.txt 2>&1
  python3 mcc_a.py submit 4
  python3 -u solve_detail.py 4 70 > logs_t04_det$i.txt 2>&1
  python3 mcc_a.py submit 4
done
echo chain04 done
