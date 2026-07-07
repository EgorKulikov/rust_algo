"""Solve drift-corrected 2px trophy (trophydc_b.npz) and test vs 95.9."""
import numpy as np
from collections import deque
import mcc
import trophy_regress as tr

N = mcc.N
GB = tr.GB
CB = tr.CB
CIRCLE = np.abs(np.sqrt((np.mgrid[0:N, 0:N][1] - N / 2) ** 2 +
                        (np.mgrid[0:N, 0:N][0] - N / 2) ** 2) - 118.5) <= 2.0
FI, FJ, MJ = tr._fi, tr._fj, tr._mj


def build(on):
    g = np.zeros((GB, GB), bool)
    g[FI[on], FJ[on]] = True
    g[FI[on], MJ[on]] = True
    return g


def cc(grid, m):
    seen = np.zeros_like(grid); out = np.zeros_like(grid)
    for i in range(GB):
        for j in range(GB):
            if grid[i, j] and not seen[i, j]:
                q = deque([(i, j)]); seen[i, j] = True; cells = [(i, j)]
                while q:
                    a, c = q.popleft()
                    for dy in (-1, 0, 1):
                        for dx in (-1, 0, 1):
                            ny, nx = a + dy, c + dx
                            if 0 <= ny < GB and 0 <= nx < GB and grid[ny, nx] and not seen[ny, nx]:
                                seen[ny, nx] = True; q.append((ny, nx)); cells.append((ny, nx))
                if len(cells) >= m:
                    for (yc, xc) in cells:
                        out[yc, xc] = True
    return out


def main(name="b"):
    d = np.load(mcc.HERE + f"/trophydc_{name}.npz")
    X = np.unpackbits(d["masks"], axis=1)[:, :tr.NR].astype(float)
    y = d["ys"].astype(float)
    M = len(y)
    Xc = X - X.mean()
    coef = np.linalg.solve(Xc.T @ Xc + 1.0 * np.eye(tr.NR), Xc.T @ (y - y.mean()))
    b = mcc.Beast(name)
    print(f"[{name}] DC M={M} coef[{coef.min():.4f},{coef.max():.4f}]", flush=True)

    def avg(grid, k=8, tag=""):
        img = CIRCLE | np.kron(grid, np.ones((CB, CB), bool))
        return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))

    best = (-1, "")
    for q in (0.55, 0.62, 0.70, 0.78):
        thr = np.quantile(coef, q)
        on = coef > thr
        g = build(on)
        for ms in (1, 8, 16):
            gg = g if ms == 1 else cc(g, ms)
            sc = avg(gg, 8, f"dcq{q}m{ms}")
            if sc > best[0]:
                best = (sc, f"q{q}/cc{ms}({gg.sum()})")
            if b.key:
                print("KEY", b.key); return
        print(f"q={q}: best {best[0]:.3f} ({best[1]})", flush=True)
    print(f"BEST {best[0]:.3f} {best[1]}", flush=True)


if __name__ == "__main__":
    main()
