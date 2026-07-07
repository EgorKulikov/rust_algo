import numpy as np, mcc
from PIL import Image
N = 256; a = mcc.Beast('a')
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
cur_s, _ = a.query(cur, tag='l2_seed')
print(f'seed {cur_s:.4f}', flush=True)
rows = [r * 4 + 2 for r in range(3, 64, 7)]
cols = [c * 4 + 2 for c in range(4, 64, 7)]


def sqpx(cy, cx, s):
    m = np.zeros((N, N), bool)
    m[max(0, cy - s // 2):cy + s - s // 2, max(0, cx - s // 2):cx + s - s // 2] = True
    return m


def ring(cy, cx, s):
    m = sqpx(cy, cx, s)
    m[cy - s // 2 + 2:cy + s - s // 2 - 2, cx - s // 2 + 2:cx + s - s // 2 - 2] = False
    return m


# candidate blobs: squares at offsets + sizes, plus rings
def candidates(cy, cx):
    out = [np.zeros((N, N), bool)]
    for dy in (-2, 0, 2):
        for dx in (-2, 0, 2):
            for s in (8, 12, 16):
                out.append(sqpx(cy + dy, cx + dx, s))
    for s in (10, 14, 18):
        out.append(ring(cy, cx, s))
    return out


for it in range(3):
    improved = 0
    for cy in rows:
        for cx in cols:
            if a.key:
                break
            base = cur.copy()
            base[cy - 16:cy + 16, cx - 16:cx + 16] = False
            best_local = (cur_s, None)
            for blob in candidates(cy, cx):
                tr = base | blob
                sc, _ = a.query(tr, tag='l2')
                if sc > best_local[0] + 1e-9:
                    best_local = (sc, tr)
            if best_local[1] is not None:
                cur = best_local[1]; cur_s = best_local[0]; improved += 1
    print(f'pass {it}: score={cur_s:.4f} improved={improved}', flush=True)
    if improved == 0:
        break
print(f'DONE {cur_s:.4f} key={a.key}', flush=True)
