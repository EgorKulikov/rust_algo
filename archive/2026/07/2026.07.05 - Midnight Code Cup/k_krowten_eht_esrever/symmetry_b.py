"""Symmetry-enhanced B solve: the trophy cup is left-right symmetric.

Symmetrize the block coefficients about x=center (average coef with its mirror),
which denoises and fills missing halves (incl. handles). Then exact circle +
symmetrized interior (thresholded, CC-denoised). Test averaged, keep best.
"""
import numpy as np
from collections import deque
import mcc

N = mcc.N
G = 64
NB = G * G
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
r = np.sqrt((xx - N / 2) ** 2 + (yy - N / 2) ** 2)
CIRCLE = np.abs(r - 118) <= 2.0

d = np.load(mcc.HERE + "/blockdata_b.npz")
X = np.unpackbits(d["masks"], axis=1)[:, :NB].astype(np.float64)
y = d["scores"].astype(np.float64)
M = len(y)
Xc = X - X.mean()
coef = np.linalg.solve(Xc.T @ Xc + 1.0 * np.eye(NB), Xc.T @ (y - y.mean())).reshape(G, G)
coef_sym = (coef + coef[:, ::-1]) / 2.0   # mirror-average about vertical axis
by, bx = np.mgrid[0:G, 0:G]
br = np.sqrt(((bx + 0.5) * 4 - N / 2) ** 2 + ((by + 0.5) * 4 - N / 2) ** 2)
print(f"M={M}", flush=True)


def cc(grid, minsz):
    seen = np.zeros_like(grid); out = np.zeros_like(grid)
    for i in range(G):
        for j in range(G):
            if grid[i, j] and not seen[i, j]:
                q = deque([(i, j)]); seen[i, j] = True; cells = [(i, j)]
                while q:
                    a, c = q.popleft()
                    for dy in (-1, 0, 1):
                        for dx in (-1, 0, 1):
                            ny, nx = a + dy, c + dx
                            if 0 <= ny < G and 0 <= nx < G and grid[ny, nx] and not seen[ny, nx]:
                                seen[ny, nx] = True; q.append((ny, nx)); cells.append((ny, nx))
                if len(cells) >= minsz:
                    for (yc, xc) in cells:
                        out[yc, xc] = True
    return out


def avg(img, k=6, tag=""):
    return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))


best = (-1, None, "")
for C, label in ((coef, "raw"), (coef_sym, "sym")):
    for q in (0.85, 0.88, 0.90):
        t = np.quantile(C, q)
        interior = (C > t) & (br < 96)
        for ms in (1, 5):
            ig = cc(interior, ms) if ms > 1 else interior
            img = CIRCLE | np.kron(ig, np.ones((4, 4), bool))
            sc = avg(img, 6, f"{label}q{q}m{ms}")
            if sc > best[0]:
                best = (sc, img.copy(), f"{label}/q{q}/cc{ms}({ig.sum()})")
            if b.key:
                print("KEY", b.key); raise SystemExit
    print(f"{label} done, best {best[0]:.3f} ({best[2]})", flush=True)
print(f"BEST {best[0]:.3f} {best[2]}", flush=True)
