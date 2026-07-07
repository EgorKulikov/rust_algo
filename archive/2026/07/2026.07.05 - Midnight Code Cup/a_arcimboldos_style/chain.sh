#!/bin/bash
# usage: chain.sh DD greedy_log
dd=$1; log=$2
until grep -q "^saved" $log 2>/dev/null; do sleep 30; done
python3 mcc_a.py submit $dd
python3 -u refine.py $dd 5 > logs_t$(printf %02d $dd)_ref.txt 2>&1
python3 mcc_a.py submit $dd
echo "chain $dd done"
