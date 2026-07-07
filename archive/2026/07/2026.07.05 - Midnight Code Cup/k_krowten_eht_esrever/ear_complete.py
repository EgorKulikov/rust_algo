"""Complete the trophy handle 'ears' by extending the observed diagonal bands.

Right ear = diagonal band x_center(y) ~ 186 - 0.69*(y-94); left = mirror. Test
symmetric completions: extend the diagonal, vary thickness, and add loop
connections to the bowl. k=8 averaging + 2x symmetry makes small gains detectable.
"""
import numpy as np
from PIL import Image
import mcc

N = mcc.N
b = mcc.Beast("b")
base = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
yy, xx = np.mgrid[0:N, 0:N]


def diag_band(y0, y1, t, slope=-0.69, x_at94=186.0):
    """right-ear diagonal band + its mirror (left)."""
    m = np.zeros((N, N), bool)
    xc = x_at94 + slope * (yy - 94)          # right center per row
    r = (np.abs(xx - xc) <= t / 2.0) & (yy >= y0) & (yy <= y1)
    xcl = (256 - x_at94) - slope * (yy - 94)  # mirror center
    l = (np.abs(xx - xcl) <= t / 2.0) & (yy >= y0) & (yy <= y1)
    return m | r | l


def avg(img, k=8, tag=""):
    return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))


print("base:", round(avg(base, 10, "eb"), 3), flush=True)
best = (-1, None, "")
cands = {}
for y0, y1 in ((90, 110), (86, 112), (88, 108)):
    for t in (3, 4, 5, 6):
        cands[f"band_{y0}-{y1}_t{t}"] = base | diag_band(y0, y1, t)
# thickened current + inward fill (toward bowl) variants
for t in (5, 7):
    band = diag_band(86, 112, t)
    cands[f"thick_t{t}"] = base | band
for _, (name, img) in enumerate(cands.items()):
    sc = avg(img, 8, "ec_" + name)
    if sc > best[0]:
        best = (sc, img.copy(), name)
    print(f"  {name}: {sc:.3f}", flush=True)
    if b.key:
        print("KEY", b.key); break
print(f"BEST {best[0]:.3f} {best[2]}", flush=True)
