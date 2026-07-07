"""Solve B's symmetric 2px trophy from trophydata_b.npz, + circle, test.

Design paints mirror-pairs together, so each coefficient is a pair's sign.
Reconstruct by painting both halves of positive-coef pairs; combine with the
fixed circle; try thresholds + component denoising; test averaged; keep best.
"""
import numpy as np
from collections import deque
import mcc
import trophy_regress as tr

N = mcc.N
GB = tr.GB
CB = tr.CB
IDX = tr.IDX
NR = tr.NR
import numpy as _np
_yy,_xx=_np.mgrid[0:tr.N,0:tr.N]
_r=_np.sqrt((_xx-tr.N/2)**2+(_yy-tr.N/2)**2)
CIRCLE = _np.abs(_r-118.5)<=2.0  # refined circle
FI, FJ, MJ = tr._fi, tr._fj, tr._mj


def build_grid(free_on):
    grid = np.zeros((GB, GB), bool)
    grid[FI[free_on], FJ[free_on]] = True
    grid[FI[free_on], MJ[free_on]] = True
    return grid


def cc(grid, minsz):
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
                if len(cells) >= minsz:
                    for (yc, xc) in cells:
                        out[yc, xc] = True
    return out


def main(name="b"):
    d = np.load(mcc.HERE + f"/trophydata_{name}.npz")
    X = np.unpackbits(d["masks"], axis=1)[:, :NR].astype(np.float64)
    y = d["scores"].astype(np.float64)
    M = len(y)
    Xc = X - X.mean()
    coef = np.linalg.solve(Xc.T @ Xc + 1.0 * np.eye(NR), Xc.T @ (y - y.mean()))
    b = mcc.Beast(name)
    print(f"[{name}] trophy M={M} NR={NR} coef[{coef.min():.4f},{coef.max():.4f}]", flush=True)

    def avg(grid, k=6, tag=""):
        img = CIRCLE | np.kron(grid, np.ones((CB, CB), bool))
        return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))

    best = (-1, "")
    for q in (0.45, 0.55, 0.62, 0.70, 0.78):
        thr = np.quantile(coef, q)
        on = coef > thr
        grid = build_grid(on)
        for ms in (1, 6, 12):
            g = grid if ms == 1 else cc(grid, ms)
            sc = avg(g, 6, f"trq{q}m{ms}")
            if sc > best[0]:
                best = (sc, f"q{q}/cc{ms}({g.sum()})")
            if b.key:
                print("KEY", b.key); return
        print(f"q={q}: best {best[0]:.3f} ({best[1]})", flush=True)
    print(f"BEST {best[0]:.3f} {best[1]}", flush=True)


if __name__ == "__main__":
    main()
