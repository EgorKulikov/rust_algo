"""Best combined B image: refined circle + fused 4px/2px trophy, edge variants.

Fuses the two independent trophy reconstructions (4px full-image regression and
2px focused regression), unions high-confidence blocks, adds the refined circle,
and tests dilation/erosion of the whole icon to catch missing edge pixels.
Heavily averaged to hunt past the key threshold (>95.955).
"""
import numpy as np
from collections import deque
from PIL import Image, ImageFilter
import mcc
import trophy_regress as tr

N = mcc.N
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
r = np.sqrt((xx - N / 2) ** 2 + (yy - N / 2) ** 2)
CIRCLE = np.abs(r - 118.5) <= 2.0

# 4px trophy from best_b
rec = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
trophy4 = rec & (r < 66)

# 2px trophy from trophydata
d = np.load(mcc.HERE + "/trophydata_b.npz")
X = np.unpackbits(d["masks"], axis=1)[:, :tr.NR].astype(float)
y = d["scores"].astype(float)
coef = np.linalg.solve((X - X.mean()).T @ (X - X.mean()) + 1.0 * np.eye(tr.NR),
                       (X - X.mean()).T @ (y - y.mean()))
on = coef > np.quantile(coef, 0.62)
g2 = np.zeros((tr.GB, tr.GB), bool)
g2[tr._fi[on], tr._fj[on]] = True
g2[tr._fi[on], tr._mj[on]] = True
trophy2 = np.kron(g2, np.ones((2, 2), bool))


def avg(img, k=10, tag=""):
    return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))


def dil(m, n):
    im = Image.fromarray((m * 255).astype("uint8"))
    for _ in range(n):
        im = im.filter(ImageFilter.MaxFilter(3))
    return np.asarray(im) > 127


def ero(m, n):
    im = Image.fromarray((m * 255).astype("uint8"))
    for _ in range(n):
        im = im.filter(ImageFilter.MinFilter(3))
    return np.asarray(im) > 127


cands = {
    "4px": CIRCLE | trophy4,
    "2px": CIRCLE | trophy2,
    "union": CIRCLE | trophy4 | trophy2,
    "inter": CIRCLE | (trophy4 & trophy2),
    "union_dil1": dil(CIRCLE | trophy4 | trophy2, 1),
    "4px_dil1": dil(CIRCLE | trophy4, 1),
    "4px_ero1": ero(CIRCLE, 0) | ero(trophy4, 1) | CIRCLE,
}
best = (-1, "")
for name, img in cands.items():
    sc = avg(img, 10, f"ult_{name}")
    print(f"{name}: white={img.mean():.4f} -> {sc:.3f}", flush=True)
    if sc > best[0]:
        best = (sc, name)
    if b.key:
        print("KEY", b.key); break
print("BEST", round(best[0], 3), best[1], flush=True)
