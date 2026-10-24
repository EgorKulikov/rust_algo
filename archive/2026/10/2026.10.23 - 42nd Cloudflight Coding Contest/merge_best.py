#!/usr/bin/env python3
"""merge_best.py <out> <prog1> <prog2> ... : per job, keep the shortest program among the inputs."""
import sys
out, srcs = sys.argv[1], sys.argv[2:]
progs = []
for s in srcs:
    blocks = [b for b in open(s).read().split('\n\n') if b.strip()]
    progs.append(blocks)
n = len(progs[0])
assert all(len(p) == n for p in progs), [len(p) for p in progs]
res = []
wins = [0]*len(srcs)
for j in range(n):
    best = min(range(len(srcs)), key=lambda i: len(progs[i][j].strip().split('\n')))
    wins[best] += 1
    res.append(progs[best][j].strip())
open(out, 'w').write('\n\n'.join(res) + '\n\n')
tot = sum(len(b.split('\n')) for b in res)
print('merged total', tot, 'wins per source', dict(zip(srcs, wins)), 'source totals', [sum(len(b.strip().split('\n')) for b in p) for p in progs])
