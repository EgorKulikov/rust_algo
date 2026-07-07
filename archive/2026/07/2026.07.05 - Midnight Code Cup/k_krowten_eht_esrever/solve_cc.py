"""Solve B from current blockdata, CC-denoise (circle+cup icon prior), test, keep best.

Recomputes ridge coefficients from all samples, then over a small grid of
(coefficient-quantile, min-component-size) builds candidate images, tests each
averaged on B, and keeps the best. Run periodically while collection grows.
Pause collection (one client per endpoint) before running.
"""
import sys
import numpy as np
from collections import deque
import mcc

G = 64
NB = G * G


def keep_large(grid, minsz):
    seen = np.zeros_like(grid); out = np.zeros_like(grid)
    for i in range(G):
        for j in range(G):
            if grid[i, j] and not seen[i, j]:
                q = deque([(i, j)]); seen[i, j] = True; cells = [(i, j)]
                while q:
                    y, x = q.popleft()
                    for dy in (-1, 0, 1):
                        for dx in (-1, 0, 1):
                            ny, nx = y + dy, x + dx
                            if 0 <= ny < G and 0 <= nx < G and grid[ny, nx] and not seen[ny, nx]:
                                seen[ny, nx] = True; q.append((ny, nx)); cells.append((ny, nx))
                if len(cells) >= minsz:
                    for (y, x) in cells:
                        out[y, x] = True
    return out


def main(name="b", k=6):
    d = np.load(mcc.HERE + f"/blockdata_{name}.npz")
    X = np.unpackbits(d["masks"], axis=1)[:, :NB].astype(np.float64)
    y = d["scores"].astype(np.float64)
    M = len(y)
    p = X.mean(); Xc = X - p; yc = y - y.mean()
    coef = np.linalg.solve(Xc.T @ Xc + 1.0 * np.eye(NB), Xc.T @ yc)
    np.save(mcc.HERE + f"/coef_{name}.npy", coef)
    order = np.argsort(coef)[::-1]
    b = mcc.Beast(name)
    print(f"[{name}] M={M} coef[{coef.min():.4f},{coef.max():.4f}]", flush=True)

    def avg(grid, tag):
        im = np.kron(grid, np.ones((4, 4), bool))
        return float(np.mean([b.query(im, tag=tag)[0] for _ in range(k)]))

    best = (-1, None, "")
    for frac in (0.10, 0.12, 0.14, 0.16, 0.18):
        g = np.zeros(NB, bool); g[order[:int(NB * frac)]] = True; g = g.reshape(G, G)
        for ms in (1, 5, 8):
            f = g if ms == 1 else keep_large(g, ms)
            sc = avg(f, f"cc{frac}_{ms}")
            tag = f"frac{frac}/cc{ms}({f.sum()})"
            print(f"  {tag} -> {sc:.3f}", flush=True)
            if sc > best[0]:
                best = (sc, f, tag)
            if b.key:
                print("KEY", b.key); return
    print(f"[{name}] BEST {best[0]:.3f} @ {best[2]}", flush=True)


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "b")
