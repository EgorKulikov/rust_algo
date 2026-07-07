import numpy as np, mcc
from PIL import Image
a = mcc.Beast('a'); N = 256
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
s0, _ = a.query(cur, tag='lt_seed'); print(f'seed {s0:.4f}', flush=True)
rows = list(range(3, 64, 7)); cols = list(range(4, 64, 7))


def ltrom(cy, cx, miss, b):
    # 2x2 grid of b-px blocks centered at (cy,cx), remove corner `miss`
    blocks = {'TL': (-b, -b), 'TR': (-b, 0), 'BL': (0, -b), 'BR': (0, 0)}
    m = np.zeros((N, N), bool)
    for k, (oy, ox) in blocks.items():
        if k == miss:
            continue
        y0, x0 = cy + oy, cx + ox
        m[max(0, y0):y0 + b, max(0, x0):x0 + b] = True
    return m


orient = {}
for cr, cc in [(r, c) for r in rows for c in cols]:
    if a.key:
        break
    cy, cx = cr * 4 + 2, cc * 4 + 2
    base = cur.copy(); base[cy - 16:cy + 16, cx - 16:cx + 16] = False
    best_v = s0; best_m = None; best_d = None
    for b in (8, 6, 10):
        for miss in ('TL', 'TR', 'BL', 'BR'):
            m = ltrom(cy, cx, miss, b)
            v, _ = a.query(base | m, tag='lt')
            if v > best_v + 1e-9:
                best_v = v; best_m = base | m; best_d = f'{miss}b{b}'
    if best_m is not None:
        cur = best_m; s0 = best_v; orient[(cr, cc)] = best_d
Image.fromarray((cur * 255).astype('uint8')).save('best_a.png')
print(f'DONE L-tromino pass: {s0:.4f} improved {len(orient)} cells key={a.key}', flush=True)
