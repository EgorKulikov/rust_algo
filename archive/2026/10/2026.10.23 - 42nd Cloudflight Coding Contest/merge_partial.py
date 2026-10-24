#!/usr/bin/env python3
"""merge_partial.py <out> <base_full_file> <partial1> [<partial2> ...]
Overlay per-job programs from partial files (written by NANO_PARTIAL) onto a full output file, keeping the shorter one."""
import sys
out, base, parts = sys.argv[1], sys.argv[2], sys.argv[3:]
blocks = [b.strip() for b in open(base).read().split('\n\n') if b.strip()]
n = len(blocks)
improved = 0; seen = 0
for pf in parts:
    try:
        txt = open(pf).read()
    except FileNotFoundError:
        continue
    for chunk in txt.split('\n\n'):
        chunk = chunk.strip()
        if not chunk.startswith('#job'):
            continue
        head, _, body = chunk.partition('\n')
        _, idx, cnt = head.split()
        idx, cnt = int(idx), int(cnt)
        lines = [l for l in body.split('\n') if l.strip()]
        if len(lines) != cnt or idx >= n:
            continue  # truncated / in-progress write
        seen += 1
        if cnt < len(blocks[idx].split('\n')):
            blocks[idx] = '\n'.join(lines); improved += 1
open(out, 'w').write('\n\n'.join(blocks) + '\n\n')
print('partial jobs seen', seen, 'improved', improved, 'total', sum(len(b.split('\n')) for b in blocks))
