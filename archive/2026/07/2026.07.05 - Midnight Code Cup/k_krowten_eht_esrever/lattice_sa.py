import numpy as np, mcc
from PIL import Image
N = 256; a = mcc.Beast('a')
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
cur_s, _ = a.query(cur, tag='sa_seed')
best = cur.copy(); best_s = cur_s
print(f'seed {cur_s:.4f}', flush=True)
rows = [r * 4 + 2 for r in range(3, 64, 7)]
cols = [c * 4 + 2 for c in range(4, 64, 7)]
# deterministic pseudo-random via index mixing (no Math.random needed; use numpy seeded)
rng = np.random.default_rng(20260705)


def sqpx(cy, cx, s):
    m = np.zeros((N, N), bool)
    if s > 0:
        m[max(0, cy - s // 2):cy + s - s // 2, max(0, cx - s // 2):cx + s - s // 2] = True
    return m


it = 0
while it < 20000 and not a.key:
    T = max(0.05, 2.0 * (1 - it / 20000))
    # pick 1-2 random cells, randomize their content
    k = 1 + int(rng.integers(2))
    trial = cur.copy()
    for _ in range(k):
        ci = int(rng.integers(len(rows))); cj = int(rng.integers(len(cols)))
        cy = rows[ci] + int(rng.integers(-3, 4)); cx = cols[cj] + int(rng.integers(-3, 4))
        trial[rows[ci] - 16:rows[ci] + 16, cols[cj] - 16:cols[cj] + 16] = False
        s = int(rng.choice([0, 6, 8, 10, 12, 14]))
        trial |= sqpx(cy, cx, s)
    ns, _ = a.query(trial, tag='sa')
    d = ns - cur_s
    if d > 0 or rng.random() < np.exp(d / T):
        cur = trial; cur_s = ns
        if cur_s > best_s:
            best = cur.copy(); best_s = cur_s
            print(f'it{it} NEW BEST {best_s:.4f} (T={T:.2f})', flush=True)
    it += 1
    if it % 500 == 0:
        print(f'it{it} cur={cur_s:.3f} best={best_s:.4f} T={T:.2f}', flush=True)
print(f'DONE best={best_s:.4f} key={a.key}', flush=True)
