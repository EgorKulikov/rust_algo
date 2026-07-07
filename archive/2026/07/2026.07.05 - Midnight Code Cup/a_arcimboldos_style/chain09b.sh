#!/bin/bash
until grep -q "seq09 done" logs_seq09.txt 2>/dev/null; do sleep 120; done
python3 -u refg.py 9 out/09.out 15 > logs_t09_refC.txt 2>&1
echo chain09b done
