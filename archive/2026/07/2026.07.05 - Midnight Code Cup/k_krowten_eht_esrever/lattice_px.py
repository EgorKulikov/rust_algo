import numpy as np, mcc
from PIL import Image
N = 256; a = mcc.Beast('a')
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
cur_s, _ = a.query(cur, tag='lp_seed')
print(f'seed {cur_s:.4f}', flush=True)
# lattice centers in pixels: cell rows 3+7k -> px (3+7k)*4+2 ; use px directly
rows = [r * 4 + 2 for r in range(3, 64, 7)]
cols = [c * 4 + 2 for c in range(4, 64, 7)]


def sqpx(cy, cx, s):
    m = np.zeros((N, N), bool)
    m[max(0, cy - s // 2):cy + s - s // 2, max(0, cx - s // 2):cx + s - s // 2] = True
    return m


SIZES = [0, 4, 6, 8, 10, 12, 14, 16, 20]
for it in range(3):
    improved = 0
    for cy in rows:
        for cx in cols:
            if a.key:
                break
            base = cur.copy()
            base[cy - 14:cy + 14, cx - 14:cx + 14] = False   # clear the 28px cell
            best_local = (cur_s, cur)
            for s in SIZES:
                tr = base.copy()
                if s > 0:
                    tr |= sqpx(cy, cx, s)
                sc, _ = a.query(tr, tag='lp')
                if sc > best_local[0] + 1e-9:
                    best_local = (sc, tr)
            if best_local[0] > cur_s + 1e-9:
                cur = best_local[1]; cur_s = best_local[0]; improved += 1
    print(f'pass {it}: score={cur_s:.4f} improved={improved}', flush=True)
    if improved == 0:
        break
print(f'DONE {cur_s:.4f} key={a.key}', flush=True)
