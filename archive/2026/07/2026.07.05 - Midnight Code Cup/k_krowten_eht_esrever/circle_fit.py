"""Fit an exact circle to B's ring and test clean circle + reconstructed cup.

The block reconstruction approximates the thin circular ring poorly (4px blocks
on a curve). A clean parametric circle outline should match far better. Estimate
center/radius from the ring blocks, then test circles of varying radius/thickness,
alone and combined with the reconstructed central cup, averaged on B.
"""
import numpy as np
import mcc

N = mcc.N
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
cx = cy = N / 2.0

# current reconstruction
from PIL import Image
rec = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
r = np.sqrt((xx - cx) ** 2 + (yy - cy) ** 2)
ring_px = rec & (r > 90)
if ring_px.sum():
    Rest = r[ring_px].mean()
else:
    Rest = 115
cup = rec & (r < 70)     # central cup blocks from reconstruction
print(f"estimated ring radius ~{Rest:.1f}, cup blocks {cup.sum()}", flush=True)


def avg(a, k=6, tag=""):
    return float(np.mean([b.query(a, tag=tag)[0] for _ in range(k)]))


def ring(R, t):
    return np.abs(r - R) <= t / 2.0


print("recon baseline    ", round(avg(rec, 6, "cf_rec"), 3), flush=True)
best = (-1, "")
for R in (Rest - 4, Rest, Rest + 4):
    for t in (3, 5, 7):
        img = ring(R, t) | cup
        sc = avg(img, 6, f"cf_R{R:.0f}t{t}")
        print(f"  ring R={R:.0f} t={t} + cup -> {sc:.3f}", flush=True)
        if sc > best[0]:
            best = (sc, f"R{R:.0f}t{t}")
        if b.key:
            print("KEY", b.key); break
print("BEST", best, flush=True)
