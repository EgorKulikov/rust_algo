import numpy as np, mcc
from itertools import product
from PIL import Image
a = mcc.Beast('a'); N = 256
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
s0, _ = a.query(cur, tag='ss_seed'); print(f'seed {s0:.4f}', flush=True)
rows = list(range(3, 64, 7)); cols = list(range(4, 64, 7))
# 15 non-empty subsets of 2x2 blocks (TL,TR,BL,BR)
POS = [('TL', -1, -1), ('TR', -1, 0), ('BL', 0, -1), ('BR', 0, 0)]
SUBSETS = [s for s in product([0, 1], repeat=4) if any(s)]


def make(cy, cx, subset, b, oy, ox):
    m = np.zeros((N, N), bool)
    for (name, by, bx), on in zip(POS, subset):
        if not on:
            continue
        y0 = cy + oy + by * b; x0 = cx + ox + bx * b
        m[max(0, y0):y0 + b, max(0, x0):x0 + b] = True
    return m


improved = 0
for cr, cc in [(r, c) for r in rows for c in cols]:
    if a.key:
        break
    cy, cx = cr * 4 + 2, cc * 4 + 2
    base = cur.copy(); base[cy - 18:cy + 18, cx - 18:cx + 18] = False
    best_v = s0; best_m = None
    for b in (8,):
        for oy, ox in [(0, 0), (2, 2), (2, -2), (-2, 2), (-2, -2)]:
            for subset in SUBSETS:
                v, _ = a.query(base | make(cy, cx, subset, b, oy, ox), tag='ss')
                if v > best_v + 1e-9:
                    best_v = v; best_m = base | make(cy, cx, subset, b, oy, ox)
    if best_m is not None:
        cur = best_m; s0 = best_v; improved += 1
        Image.fromarray((cur * 255).astype('uint8')).save('best_a.png')
        print(f'  cell({cr},{cc}) -> {s0:.4f}', flush=True)
Image.fromarray((cur * 255).astype('uint8')).save('best_a.png')
print(f'DONE 15-subset pass: {s0:.4f} improved {improved} cells key={a.key}', flush=True)
