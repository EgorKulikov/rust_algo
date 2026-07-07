import numpy as np, mcc
from PIL import Image
a = mcc.Beast('a'); N = 256
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
s0, _ = a.query(cur, tag='tr_seed'); print(f'seed {s0:.4f}', flush=True)
# 6 fixed trominoes as block-offsets
TROM = {
    'L1': [(0, 0), (0, 1), (1, 0)],
    'L2': [(0, 0), (0, 1), (1, 1)],
    'L3': [(0, 0), (1, 0), (1, 1)],
    'L4': [(0, 1), (1, 0), (1, 1)],
}
rows = list(range(3, 64, 7)); cols = list(range(4, 64, 7))


def make(cy, cx, cells, b):
    # cells in block units; center the shape on (cy,cx); block size b px
    ys = [y for y, x in cells]; xs = [x for y, x in cells]
    hy = (max(ys) + 1) * b; wx = (max(xs) + 1) * b
    oy = cy - hy // 2; ox = cx - wx // 2
    m = np.zeros((N, N), bool)
    for (yy, xx) in cells:
        y0 = oy + yy * b; x0 = ox + xx * b
        if 0 <= y0 < N - b and 0 <= x0 < N - b:
            m[y0:y0 + b, x0:x0 + b] = True
    return m


for cr, cc in [(r, c) for r in rows for c in cols]:
    if a.key:
        break
    cy, cx = cr * 4 + 2, cc * 4 + 2
    base = cur.copy(); base[cy - 16:cy + 16, cx - 16:cx + 16] = False
    best_v = s0; best_m = None
    for name, cells in TROM.items():
        for b in (6, 8, 10, 12):
            m = make(cy, cx, cells, b)
            v, _ = a.query(base | m, tag='tr')
            if v > best_v + 1e-9:
                best_v = v; best_m = base | m
    if best_m is not None:
        cur = best_m; s0 = best_v
Image.fromarray((cur * 255).astype('uint8')).save('best_a.png')
print(f'DONE all-tromino pass: {s0:.4f} key={a.key}', flush=True)
