"""Decompose best_b into circle/trophy, measure each, fine-tune the circle."""
import numpy as np
from PIL import Image
import mcc

N = mcc.N
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
r = np.sqrt((xx - N / 2) ** 2 + (yy - N / 2) ** 2)
rec = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
trophy = rec & (r < 65)          # trophy part of current best
print(f"trophy px {trophy.sum()}", flush=True)


def avg(img, k=8, tag=""):
    return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))


print("trophy alone   ", round(avg(trophy, 8, "cf_troalone"), 3), flush=True)
best = (-1, "")
for R in (117.0, 117.5, 118.0, 118.5, 119.0):
    for ht in (1.5, 2.0, 2.5, 3.0):
        ring = np.abs(r - R) <= ht
        sc = avg(ring, 8, f"cf_ringonly_R{R}h{ht}")
        # also combined with trophy
        sc2 = avg(ring | trophy, 8, f"cf_R{R}h{ht}")
        if sc2 > best[0]:
            best = (sc2, f"R{R}ht{ht} ({ring.sum()}px)")
        print(f"R={R} ht={ht}: ring-alone={sc:.3f} ring+trophy={sc2:.3f}", flush=True)
        if b.key:
            print("KEY", b.key); raise SystemExit
print("BEST", round(best[0], 3), best[1], flush=True)
