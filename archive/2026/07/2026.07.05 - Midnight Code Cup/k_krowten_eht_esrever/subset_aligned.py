import numpy as np, mcc
from itertools import product
from PIL import Image
a = mcc.Beast('a'); N = 256
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
s0, _ = a.query(cur, tag='sa_seed'); print(f'seed {s0:.4f}', flush=True)
rows = list(range(3, 64, 7)); cols = list(range(4, 64, 7))
SUBSETS = [s for s in product([0, 1], repeat=4) if any(s)]  # TL,TR,BL,BR


def grid_tl(cy, cx):
    return int(round((cy - 8) / 8) * 8), int(round((cx - 8) / 8) * 8)


def make(gy, gx, subset):
    m = np.zeros((N, N), bool)
    for (by, bx), on in zip([(0, 0), (0, 1), (1, 0), (1, 1)], subset):
        if on:
            y0, x0 = gy + by * 8, gx + bx * 8
            m[max(0, y0):y0 + 8, max(0, x0):x0 + 8] = True
    return m


improved = 0
for cr, cc in [(r, c) for r in rows for c in cols]:
    if a.key:
        break
    cy, cx = cr * 4 + 2, cc * 4 + 2
    gy, gx = grid_tl(cy, cx)
    base = cur.copy(); base[gy:gy + 16, gx:gx + 16] = False
    best_v = -1; best_m = None
    for subset in SUBSETS:
        m = base | make(gy, gx, subset)
        v, _ = a.query(m, tag='sa')
        if v > best_v:
            best_v = v; best_m = m
    if best_v > s0 + 1e-9:
        improved += 1
        print(f'  cell({cr},{cc}) {s0:.4f}->{best_v:.4f}', flush=True)
    cur = best_m; s0 = best_v
    Image.fromarray((cur * 255).astype('uint8')).save('best_a.png')
print(f'DONE aligned 15-subset: {s0:.4f} improved {improved} key={a.key}', flush=True)
