"""Solve edge corrections from edgedata_b.npz: flip beneficial boundary blocks."""
import numpy as np
import mcc
import edge_regress as er

N = mcc.N
GB = er.GB
CB = er.CB
BASE = np.load(mcc.HERE + "/base_b.npy")
FI, FJ, MJ, NR = er.FI, er.FJ, er.MJ, er.NR


def flipped(on):
    grid = np.zeros((GB, GB), bool)
    grid[FI[on], FJ[on]] = True
    grid[FI[on], MJ[on]] = True
    return BASE ^ np.kron(grid, np.ones((CB, CB), bool))


def main(name="b"):
    d = np.load(mcc.HERE + f"/edgedata_{name}.npz")
    X = np.unpackbits(d["masks"], axis=1)[:, :NR].astype(float)
    y = d["scores"].astype(float)
    M = len(y)
    Xc = X - X.mean()
    coef = np.linalg.solve(Xc.T @ Xc + 1.0 * np.eye(NR), Xc.T @ (y - y.mean()))
    b = mcc.Beast(name)
    print(f"[{name}] edge M={M} NR={NR} coef[{coef.min():.4f},{coef.max():.4f}]", flush=True)

    def avg(img, k=8, tag=""):
        return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))

    print("base   ", round(avg(BASE, 8, "es_base"), 3), flush=True)
    best = (-1, "")
    for thr in (0.0, np.quantile(coef, 0.90), np.quantile(coef, 0.95), np.quantile(coef, 0.98)):
        on = coef > thr
        img = flipped(on)
        sc = avg(img, 8, f"es_thr{thr:.4f}")
        print(f"flip {on.sum()} blocks (thr={thr:.4f}) white={img.mean():.4f} -> {sc:.3f}", flush=True)
        if sc > best[0]:
            best = (sc, on.sum())
        if b.key:
            print("KEY", b.key); return
    print(f"BEST {best[0]:.3f} ({best[1]} flips)", flush=True)


if __name__ == "__main__":
    main()
