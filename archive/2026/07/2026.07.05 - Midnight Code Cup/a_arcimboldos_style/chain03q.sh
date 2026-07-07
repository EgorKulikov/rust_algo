#!/bin/bash
until grep -q "^final err" logs_t03_prune2.txt 2>/dev/null; do sleep 60; done
tail -n 2 logs_t03_prune2.txt
new=$(python3 -c "
import sys; sys.path.insert(0,'.')
import mcc_a
print(mcc_a.score_file(3, 'out/03q.out'))")
cur=$(python3 -c "
import sys; sys.path.insert(0,'.')
import mcc_a
print(mcc_a.score_file(3, 'out/03.out'))")
echo "pruned=$new current=$cur"
if [ "$new" -lt "$cur" ]; then cp out/03q.out out/03.out; echo promoted; fi
echo chain03q done
