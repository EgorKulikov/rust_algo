"""Small-perturbation regression around best_a to find joint improvements.

A is deterministic and gives signal only for small white changes (gates above
~15%). Greedy tested single-cell flips and found none improve. But a nonlinear/
gated objective can have JOINT improvements (cell i helps only if j also flips)
that single-flip greedy misses. We flip small random subsets of 4px cells around
best_a, regress the exact score deltas -> per-cell marginal effect, then apply
the beneficial (positive-coef) flips in batches and keep any that improve.
"""
import numpy as np
from PIL import Image
import mcc

N = mcc.N
CELL = 4
G = N // CELL          # 64
NB = G * G             # 4096
a = mcc.Beast("a")
base = np.asarray(Image.open(mcc.HERE + "/best_a.png").convert("L")) > 127
b0, _ = a.query(base, tag="sa_base")
print(f"base score {b0:.4f}, white {base.mean():.4f}", flush=True)


def apply_flips(cells):
    img = base.copy()
    for c in cells:
        i, j = c // G, c % G
        img[i * CELL:(i + 1) * CELL, j * CELL:(j + 1) * CELL] ^= True
    return img


# collect small-perturbation samples
M, S = 3000, 12
rng = np.random.default_rng(0)
rows, ys = [], []
for t in range(M):
    if a.key:
        break
    cells = rng.choice(NB, S, replace=False)
    s, _ = a.query(apply_flips(cells), tag=f"sa{t}")
    rows.append(cells); ys.append(s - b0)
    if (t + 1) % 500 == 0:
        print(f"  collected {t+1}/{M}", flush=True)
# build sparse design and regress
X = np.zeros((len(ys), NB))
for r, cells in enumerate(rows):
    X[r, cells] = 1.0
y = np.array(ys)
Xc = X - X.mean(0)
coef = np.linalg.solve(Xc.T @ Xc + 0.5 * np.eye(NB), Xc.T @ (y - y.mean()))
order = np.argsort(coef)[::-1]
print(f"coef range [{coef.min():.4f},{coef.max():.4f}], #positive={np.sum(coef>0)}", flush=True)
# apply beneficial flips in growing batches (deterministic -> exact)
best = (b0, 0)
for nb in (3, 6, 10, 20, 40, 80, 150):
    cells = order[:nb]
    s, k = a.query(apply_flips(cells), tag=f"sa_apply{nb}")
    print(f"  flip top-{nb} positive-coef cells -> {s:.4f}", flush=True)
    if s > best[0]:
        best = (s, nb)
    if k:
        print("KEY", k); break
print(f"BEST {best[0]:.4f} (top-{best[1]} flips) vs base {b0:.4f}", flush=True)
