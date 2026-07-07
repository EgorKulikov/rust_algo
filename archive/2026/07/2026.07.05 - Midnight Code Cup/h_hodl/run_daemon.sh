#!/bin/bash
# Supervisor: auto-respawn, absolute-path cmdline (immune to `pkill -xf "python3 daemon.py"`).
cd "$(dirname "$0")"
mkdir -p work
while true; do
  python3 /home/egor/proj/rust_algo/tasks/h/daemon.py >> work/daemon.out 2>&1
  echo "$(date -u '+%F %T') daemon exited rc=$?, respawn in 3s" >> work/daemon.out
  sleep 3
done
