#!/usr/bin/env python3
"""Navigation + capture helpers for the expedition spaceship API."""
import json
import subprocess
import sys
import time
import math

TOK = 'Authorization: Bearer ur2abyvret836ne7:165'
API = 'https://expedition.midnightcodecup.org/api/spaceship'


def req(path, post=False):
    cmd = ['curl', '-s', '-H', TOK]
    if post:
        cmd += ['-X', 'POST']
    cmd.append(API + path)
    for attempt in range(5):
        out = subprocess.run(cmd, capture_output=True, text=True).stdout
        try:
            return json.loads(out)
        except json.JSONDecodeError:
            time.sleep(1)
    raise RuntimeError(f'request failed: {path}: {out[:200]}')


def state():
    return req('')


def set_acc(ax, ay):
    return req(f'/acceleration?ax={ax}&ay={ay}', post=True)


def stop():
    return req('/stop', post=True)


def wait_stopped():
    while True:
        s = state()
        v = s['velocity']
        if abs(v['x']) < 0.01 and abs(v['y']) < 0.01:
            return s
        time.sleep(1)


def goto(tx, ty, tol=30):
    """Bang-bang per-axis-ish: accelerate toward target, flip at halfway, stop."""
    while True:
        s = wait_stopped()
        px, py = s['position']['x'], s['position']['y']
        dx, dy = tx - px, ty - py
        dist = math.hypot(dx, dy)
        if dist < tol:
            return s
        # accelerate toward target for half the distance, then brake
        ux, uy = dx / dist, dy / dist
        set_acc(10 * ux, 10 * uy)
        t_half = math.sqrt(dist / 10)
        time.sleep(t_half)
        stop()


def view(fname, tool=None):
    q = f'?tool={tool}' if tool else ''
    cmd = ['curl', '-s', '-H', TOK, '-D', fname + '.hdr', '-o', fname, API + '/view' + q]
    subprocess.run(cmd, check=True)
    hdr = open(fname + '.hdr').read()
    pos = {}
    for line in hdr.splitlines():
        low = line.lower()
        for k in ('position-x', 'position-y', 'updated-at-millis'):
            if low.startswith(f'x-spaceship-{k}:'):
                pos[k] = float(line.split(':', 1)[1])
    return pos


if __name__ == '__main__':
    if sys.argv[1] == 'goto':
        s = goto(float(sys.argv[2]), float(sys.argv[3]))
        print(json.dumps(s))
    elif sys.argv[1] == 'state':
        print(json.dumps(state()))
    elif sys.argv[1] == 'view':
        print(view(sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else None))
    elif sys.argv[1] == 'capture':
        # capture N frames every DT seconds: nav.py capture prefix N DT
        prefix, n, dt = sys.argv[2], int(sys.argv[3]), float(sys.argv[4])
        meta = []
        for i in range(n):
            t_start = time.time()
            info = view(f'{prefix}{i}.png')
            meta.append(info)
            lag = time.time() - t_start
            if lag < dt:
                time.sleep(dt - lag)
        with open(f'{prefix}meta.json', 'w') as f:
            json.dump(meta, f)
        print(f'captured {n} frames -> {prefix}*.png')
