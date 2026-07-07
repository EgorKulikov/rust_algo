"""Replace the block-quantized bowl with a pixel-exact smoothed bowl.

Fit best_b's bowl left/right edges vs row to smooth curves, re-fill between them
at 1px resolution (kills the 4px staircase = the block trap). Enforce symmetry
(edges mirror about x=128). Test small symmetric edge offsets, keep best.
"""
import numpy as np
from PIL import Image
import mcc

N = mcc.N
b = mcc.Beast("b")
base = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
CX = 128
R0, R1 = 70, 123          # bowl rows

# extract per-row bowl half-width (symmetric): rightmost white minus center, in the bowl band
rows = np.arange(R0, R1)
halfw = []
for y in rows:
    xs = np.where(base[y, CX:170])[0]     # right side of bowl
    halfw.append(xs.max() if len(xs) else 0)
halfw = np.array(halfw, float)
# smooth with a low-order polynomial fit
good = halfw > 4
coef = np.polyfit(rows[good], halfw[good], 4)
smooth_hw = np.polyval(coef, rows)


def build(offset):
    img = base.copy()
    img[R0:R1, 88:169] = False          # clear old bowl region (keep ears outside x168)
    yy, xx = np.mgrid[0:N, 0:N]
    for i, y in enumerate(rows):
        hw = smooth_hw[i] + offset
        if hw <= 3:
            continue
        img[y, int(CX - hw):int(CX + hw) + 1] = True
    # restore ears (they were outside 88..169 mostly; also re-OR original ear pixels)
    ear = base & ((np.mgrid[0:N, 0:N][1] < 88) | (np.mgrid[0:N, 0:N][1] > 168))
    return img | ear


def avg(img, k=8, tag=""):
    return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))


print("base:", round(avg(base, 10, "sbb"), 3), flush=True)
best = (-1, None, "")
for off in (-2, -1, 0, 1, 2):
    img = build(off)
    sc = avg(img, 8, f"sb_off{off}")
    if sc > best[0]:
        best = (sc, img.copy(), f"offset{off} white={img.mean():.4f}")
    print(f"  offset {off}: {sc:.3f}", flush=True)
    if b.key:
        print("KEY", b.key); break
print(f"BEST {best[0]:.3f} {best[2]}", flush=True)
