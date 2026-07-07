#!/bin/bash
until grep -q "^saved" logs_t04_resume.txt 2>/dev/null; do sleep 60; done
python3 -u refg.py 4 out/04g.out 10 > logs_t04g_ref.txt 2>&1
echo chain04g done
