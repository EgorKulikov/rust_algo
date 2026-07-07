"""Denoise B's block reconstruction with the connected-icon prior and test.

B's target is a connected icon (circle ring + central cup). The regression's
scattered isolated blocks are noise; keeping only large 8-connected components
should remove them and raise the score. Tests raw top-k vs CC-filtered.
"""
import numpy as np
from collections import deque
import mcc

G = 64
b = mcc.Beast("b")
coef = np.load(mcc.HERE + "/coef_b.npy")
order = np.argsort(coef)[::-1]


def img_from_grid(grid):
    return np.kron(grid, np.ones((4, 4), bool))


def avg(grid, k=5, tag=""):
    im = img_from_grid(grid)
    return float(np.mean([b.query(im, tag=tag)[0] for _ in range(k)]))


def keep_large(grid, minsz):
    seen = np.zeros_like(grid)
    out = np.zeros_like(grid)
    for i in range(G):
        for j in range(G):
            if grid[i, j] and not seen[i, j]:
                q = deque([(i, j)])
                seen[i, j] = True
                cells = [(i, j)]
                while q:
                    y, x = q.popleft()
                    for dy in (-1, 0, 1):
                        for dx in (-1, 0, 1):
                            ny, nx = y + dy, x + dx
                            if 0 <= ny < G and 0 <= nx < G and grid[ny, nx] and not seen[ny, nx]:
                                seen[ny, nx] = True
                                q.append((ny, nx))
                                cells.append((ny, nx))
                if len(cells) >= minsz:
                    for (y, x) in cells:
                        out[y, x] = True
    return out


for frac in (0.10, 0.12, 0.14, 0.16):
    g = np.zeros(4096, bool)
    g[order[:int(4096 * frac)]] = True
    g = g.reshape(G, G)
    raw = avg(g, 5, "raw")
    line = f"frac {frac}: raw({g.sum()})={raw:.3f}"
    for ms in (3, 5, 8):
        f = keep_large(g, ms)
        line += f" | CC>={ms}({f.sum()})={avg(f, 5, 'cc'):.3f}"
    print(line, flush=True)
    if b.key:
        print("KEY", b.key)
        break
