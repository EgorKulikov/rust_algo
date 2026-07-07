import numpy as np, mcc
from PIL import Image
N = 256; G = 64; a = mcc.Beast('a')
cur = (np.asarray(Image.open('best_a.png').convert('L')) > 127).reshape(G, 4, G, 4).mean(axis=(1, 3)) > 0.5
cur_s, _ = a.query(np.kron(cur, np.ones((4, 4), bool)), tag='lr_seed')
print(f'seed {cur_s:.4f}', flush=True)
rows = list(range(3, 64, 7)); cols = list(range(4, 64, 7))


def sq(s):
    o = s // 2; return [(dy, dx) for dy in range(-o, s - o) for dx in range(-o, s - o)]


SHAPES = [[], sq(1), sq(2), sq(3), sq(4),
          [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)],
          [(0, 0), (0, 1), (0, -1), (-1, 0), (-1, 1), (-1, -1)],
          [(dy, dx) for dy in range(-1, 2) for dx in range(-2, 3)],
          [(dy, dx) for dy in range(-2, 3) for dx in range(-1, 2)]]


def toimg(c):
    return np.kron(c, np.ones((4, 4), bool))


for it in range(4):
    improved = 0
    for r in rows:
        for c in cols:
            if a.key:
                break
            base = cur.copy()
            base[max(0, r - 3):r + 4, max(0, c - 3):c + 4] = False
            best_local = (cur_s, cur)
            for shp in SHAPES:
                tr = base.copy()
                for dy, dx in shp:
                    y, x = r + dy, c + dx
                    if 0 <= y < G and 0 <= x < G:
                        tr[y, x] = True
                s, _ = a.query(toimg(tr), tag='lr')
                if s > best_local[0] + 1e-9:
                    best_local = (s, tr)
            if best_local[0] > cur_s + 1e-9:
                cur = best_local[1]; cur_s = best_local[0]; improved += 1
    print(f'pass {it}: score={cur_s:.4f} improved={improved}', flush=True)
    if improved == 0:
        break
print(f'DONE {cur_s:.4f} key={a.key}', flush=True)
