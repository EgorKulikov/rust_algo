#!/bin/bash
python3 -u solve_detail.py 9 260 8 0 > logs_t09_detfast.txt 2>&1
python3 -u refine.py 9 4 > logs_t09_refA.txt 2>&1
python3 -u solve_detail.py 9 400 12 0 > logs_t09_detB.txt 2>&1
python3 -u refine.py 9 3 > logs_t09_refB.txt 2>&1
echo seq09 done
