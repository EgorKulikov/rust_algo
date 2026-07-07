#!/bin/bash
# Supervisor: auto-respawn the task-I daemon if it exits for any reason.
# Absolute-path cmdline so task B's `pkill -xf "python3 daemon.py"` can't hit us.
cd "$(dirname "$0")"
while true; do
    python3 /home/egor/proj/rust_algo/tasks/i/daemon.py >> work/daemon.out 2>&1
    echo "[$(date -u +%H:%M:%S)] [supervisor] daemon exited (rc=$?), respawning in 5s" >> work/daemon.log
    sleep 5
done
