"""Systematic interleaved feature scan for B.

Scan symmetric blocks across the trophy/inner region; for each, test adding
(and removing) white via drift-canceled interleaved pairs. Keep only confirmed
improvements (>3 SE, >0.02). Same reliable method that found the ears (+0.19),
applied everywhere. Symmetric (2x signal). Updates best_b as features are found.
"""
import numpy as np
from PIL import Image
import mcc

N = mcc.N
CX = 128
b = mcc.Beast("b")
cur = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
BS = 6                       # block size
# region to scan (trophy + inner ring area), left half only (mirror)
YS = range(56, 190, BS)
XS = range(56, CX, BS)       # left half


def symblock(y, x):
    m = np.zeros((N, N), bool)
    m[y:y + BS, x:x + BS] = True
    return m | m[:, ::-1]


def compare(cand, k=8):
    d = []
    for _ in range(k):
        s1 = b.query(cur, tag="fs_b")[0]
        s2 = b.query(cand, tag="fs_c")[0]
        d.append(s2 - s1)
    d = np.array(d)
    return d.mean(), d.std() / np.sqrt(len(d))


for pass_i in range(3):
    if b.key:
        break
    accepted = 0
    for y in YS:
        for x in XS:
            if b.key:
                break
            blk = symblock(y, x)
            # skip if block already fully matches (all white or would duplicate)
            add = cur | blk
            if not np.array_equal(add, cur):
                m, se = compare(add, 8)
                if m > 3 * se and m > 0.02:
                    cur = add
                    Image.fromarray((cur * 255).astype("uint8")).save(mcc.HERE + "/best_b.png")
                    accepted += 1
                    print(f"+ADD ({y},{x}): {m:+.3f} accepted", flush=True)
                    continue
            rem = cur & ~blk
            if not np.array_equal(rem, cur):
                m, se = compare(rem, 8)
                if m > 3 * se and m > 0.02:
                    cur = rem
                    Image.fromarray((cur * 255).astype("uint8")).save(mcc.HERE + "/best_b.png")
                    accepted += 1
                    print(f"-REM ({y},{x}): {m:+.3f} accepted", flush=True)
    print(f"pass {pass_i}: accepted {accepted}", flush=True)
    if accepted == 0:
        break
print("DONE", flush=True)
