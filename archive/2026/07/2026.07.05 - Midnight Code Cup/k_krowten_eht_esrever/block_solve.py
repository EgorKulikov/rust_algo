"""Solve for B's target block signs from blockdata_<name>.npz and test it.

Regression: y = base + a * X @ s, X in {0,1}^(M x 4096) block indicators.
Center design (X - p) so columns ~orthogonal to intercept; least-squares gives
coefficients proportional to s_b. Positive coeff -> target-white block. Build the
image (optionally thresholded at various quantiles), query it (averaged) on B.
"""
import sys
import os
import numpy as np
import mcc

N = mcc.N
C = 4
G = N // C
NB = G * G


def img_from_bits(bits):
    return np.kron(bits.reshape(G, G), np.ones((C, C), bool))


def solve(name, test=True):
    d = np.load(os.path.join(mcc.HERE, f"blockdata_{name}.npz"))
    masks = np.unpackbits(d["masks"], axis=1)[:, :NB].astype(np.float64)
    y = d["scores"].astype(np.float64)
    M = len(y)
    print(f"[{name}] {M} samples, white% mean {masks.mean()*100:.1f}")
    p = masks.mean()
    Xc = masks - p
    yc = y - y.mean()
    # ridge-regularized least squares for stability
    A = Xc.T @ Xc + 1e-3 * np.eye(NB)
    coef = np.linalg.solve(A, Xc.T @ yc)
    order = np.argsort(coef)[::-1]
    print(f"[{name}] coef range [{coef.min():.4f},{coef.max():.4f}]")
    beast = mcc.Beast(name)

    def avg(bits, k=6, tag=""):
        img = img_from_bits(bits)
        return float(np.mean([beast.query(img, tag=tag)[0] for _ in range(k)]))

    if test:
        # try several thresholds on the coefficient (how many blocks to whiten)
        for frac in (0.08, 0.10, 0.12, 0.14, 0.16, 0.20):
            nb = int(NB * frac)
            bits = np.zeros(NB, bool)
            bits[order[:nb]] = True
            sc = avg(bits, 6, tag=f"sol{frac}")
            print(f"[{name}] top {frac*100:.0f}% ({nb} blocks) -> {sc:.3f}", flush=True)
            if beast.key:
                print("KEY", beast.key)
                break
    np.save(os.path.join(mcc.HERE, f"coef_{name}.npy"), coef)
    return coef


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "b"
    solve(name)
