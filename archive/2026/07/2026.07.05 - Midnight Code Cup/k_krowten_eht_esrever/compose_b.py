"""Comprehensive B solve: exact circle + reconstructed interior, keep best.

Recompute ridge coef from all samples. Replace the noisy ring with a clean
circle (R=118,t=4). For the interior (r<95) and outer band (potential text),
try coefficient thresholds and connected-component denoising; test each averaged,
keep best. Saves the winning image as best via mcc's tracking.
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
p = X.mean(); Xc = X - p
coef = np.linalg.solve(Xc.T @ Xc + 1.0 * np.eye(NB), Xc.T @ (y - y.mean()))
print(f"M={M} coef[{coef.min():.4f},{coef.max():.4f}]", flush=True)
Cg = coef.reshape(G, G)
# per-block center radius
by, bx = np.mgrid[0:G, 0:G]
br = np.sqrt(((bx + 0.5) * 4 - N / 2) ** 2 + ((by + 0.5) * 4 - N / 2) ** 2)


def blocks_to_img(grid):
    return np.kron(grid, np.ones((4, 4), bool))


def cc(grid, minsz):
    seen = np.zeros_like(grid); out = np.zeros_like(grid)
    for i in range(G):
        for j in range(G):
            if grid[i, j] and not seen[i, j]:
                q = deque([(i, j)]); seen[i, j] = True; cells = [(i, j)]
                while q:
                    yy2, xx2 = q.popleft()
                    for dy in (-1, 0, 1):
                        for dx in (-1, 0, 1):
                            ny, nx = yy2 + dy, xx2 + dx
                            if 0 <= ny < G and 0 <= nx < G and grid[ny, nx] and not seen[ny, nx]:
                                seen[ny, nx] = True; q.append((ny, nx)); cells.append((ny, nx))
                if len(cells) >= minsz:
                    for (yc, xc) in cells:
                        out[yc, xc] = True
    return out


def avg(img, k=6, tag=""):
    return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))


thr = np.quantile(coef, 0.85)
best = (-1, None, "")
for q in (0.83, 0.86, 0.89):
    t = np.quantile(coef, q)
    interior = (Cg > t) & (br < 95 / 4 * 4)  # interior blocks
    interior = (Cg > t) & (br < 96)
    for ms in (1, 4, 8):
        ig = cc(interior, ms) if ms > 1 else interior
        img = CIRCLE | blocks_to_img(ig)
        sc = avg(img, 6, f"cmp_q{q}m{ms}")
        if sc > best[0]:
            best = (sc, img.copy(), f"q{q}/cc{ms}(int={ig.sum()})")
        if b.key:
            print("KEY", b.key); raise SystemExit
    print(f"q={q}: tested, best {best[0]:.3f} ({best[2]})", flush=True)
# try adding outer band (potential text) to the best interior
print(f"BEST {best[0]:.3f} {best[2]}", flush=True)
