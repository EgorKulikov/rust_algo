import numpy as np, mcc
from itertools import product
from PIL import Image
a = mcc.Beast('a'); N = 256
cur = np.asarray(Image.open('best_a.png').convert('L')) > 127
cur_s, _ = a.query(cur, tag='fc_seed'); print(f'seed {cur_s:.4f}', flush=True)
SUBSETS = [s for s in product([0, 1], repeat=4) if any(s)]


def make(ri, ci, subset):
    m = np.zeros((N, N), bool); ty, tx = 8 + 28 * ri, 8 + 28 * ci
    for (by, bx), on in zip([(0, 0), (0, 1), (1, 0), (1, 1)], subset):
        if on:
            m[ty + by * 8:ty + by * 8 + 8, tx + bx * 8:tx + bx * 8 + 8] = True
    return m


for ri in range(9):
    for ci in range(9):
        if a.key:
            break
        base = cur.copy(); base[28 * ri:28 * ri + 28, 28 * ci:28 * ci + 28] = False
        best_v = -1; best_m = None
        for subset in SUBSETS:
            m = base | make(ri, ci, subset)
            v, _ = a.query(m, tag='fc')
            if v > best_v:
                best_v = v; best_m = m
        cur = best_m; cur_s = best_v
    Image.fromarray((cur * 255).astype('uint8')).save('best_a.png')
    print(f'row {ri} done: {cur_s:.4f}', flush=True)
print(f'DONE force-clean: {cur_s:.4f} key={a.key}', flush=True)
