import numpy as np, mcc
from itertools import product
from PIL import Image
a = mcc.Beast('a'); N = 256
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
cur_s, _ = a.query(cur, tag='cs_seed'); print(f'seed {cur_s:.4f}', flush=True)
rows = list(range(3, 64, 7)); cols = list(range(4, 64, 7))
SUBSETS = [s for s in product([0, 1], repeat=4) if any(s)]


def make(cy, cx, subset):
    # TL of 2x2 grid at (cy-6, cx-10); each block 8px
    m = np.zeros((N, N), bool)
    gy, gx = cy - 6, cx - 10
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
    base = cur.copy(); base[cy - 14:cy + 14, cx - 14:cx + 14] = False
    best_v = cur_s; best_m = cur  # keep original unless beaten
    for subset in SUBSETS:
        m = base | make(cy, cx, subset)
        v, _ = a.query(m, tag='cs')
        if v > best_v + 1e-9:
            best_v = v; best_m = m
    if best_v > cur_s + 1e-9:
        improved += 1
        print(f'  cell({cr},{cc}) {cur_s:.4f}->{best_v:.4f}', flush=True)
        cur = best_m; cur_s = best_v
        Image.fromarray((cur * 255).astype('uint8')).save('best_a.png')
print(f'DONE correct-subset: {cur_s:.4f} improved {improved} key={a.key}', flush=True)
