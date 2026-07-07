#!/usr/bin/env python3
"""Per-stock status table: ours/best profits, hourly deltas, score. Run from tasks/h.

Prints to stdout AND overwrites work/status_table.txt each run.
"""
import io
import json
import sys
import time

_buf = io.StringIO()


class _Tee:
    def write(self, s):
        _buf.write(s)
        sys.__stdout__.write(s)

    def flush(self):
        sys.__stdout__.flush()


sys.stdout = _Tee()
print(time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime()))

snaps = [json.loads(l) for l in open('work/standings.jsonl')]
b = snaps[-1]
a = min(snaps, key=lambda s: abs(s['ts'] - (b['ts'] - 3600)))
dt_min = (b['ts'] - a['ts']) / 60 or 1
NAME = {1: '6/7 scalp', 6: 'TSLA', 12: 'iid<990', 13: 'NVDA', 15: 'BABA-HK', 16: 'ASML',
        20: 'sine', 27: 'VW', 29: 'SoftBank', 36: 'momentum', 44: 'AMC', 45: 'NOKIA',
        47: 'BTC', 49: 'GME', 50: 'Nintendo', 5: 'monthly?', 35: 'monthly?', 19: '?',
        43: '?', 26: 'glitchy?', 48: '?'}
ROUND = {2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 21, 22, 23, 24, 25, 28, 30, 31, 32, 33,
         34, 37, 38, 39, 40, 41, 42, 46}


def fmt(v):
    av = abs(v)
    if av >= 1e9: return f'{v/1e9:.2f}B'
    if av >= 1e6: return f'{v/1e6:.1f}M'
    if av >= 1e3: return f'{v/1e3:.0f}k'
    return f'{v:.0f}'


print(f'window: {dt_min:.0f} min   (rank {a["us"]["rank"]}era{b["us"]["rank"]}, '
      f'total {a["us"]["total"]:.0f}era{b["us"]["total"]:.0f})'.replace('era', '→'))
print(f'{"id":>3} {"model":<10} {"ours":>8} {"Δours/h":>8} {"best":>8} {"Δbest/h":>8} {"score":>6} {"Δscore":>7}')
tot = tot_old = 0
rows = []
for i in range(1, 51):
    s = str(i)
    b1 = float(b['best'].get(s) or 0); b0 = float(a['best'].get(s) or 0)
    o1 = float(b['us']['cells'][i - 1] or 0); o0 = float(a['us']['cells'][i - 1] or 0)
    sc1 = 48 * max(0, o1) / b1 if b1 > 0 else 0
    sc0 = 48 * max(0, o0) / b0 if b0 > 0 else 0
    tot += sc1; tot_old += sc0
    k = 60 / dt_min
    rows.append((i, NAME.get(i, 'round' if i in ROUND else '?'), o1, (o1 - o0) * k,
                 b1, (b1 - b0) * k, sc1, sc1 - sc0))
for r in rows:  # sorted by stock id
    print(f'{r[0]:>3} {r[1]:<10} {fmt(r[2]):>8} {fmt(r[3]):>8} {fmt(r[4]):>8} {fmt(r[5]):>8} {r[6]:>6.1f} {r[7]:>+7.1f}')
print(f'\nTOTAL score {tot:.1f} ({tot - tot_old:+.1f} in window)')

with open('work/status_table.txt', 'w') as f:
    f.write(_buf.getvalue())
