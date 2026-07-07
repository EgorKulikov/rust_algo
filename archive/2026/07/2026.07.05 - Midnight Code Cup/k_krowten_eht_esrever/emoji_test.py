"""Test cup/trophy emoji silhouettes as B's central shape + fixed circle."""
import glob
import os
import numpy as np
from PIL import Image
import mcc

N = mcc.N
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
r = np.sqrt((xx - N / 2) ** 2 + (yy - N / 2) ** 2)
CIRCLE = np.abs(r - 118.5) <= 2.0

# trophy region from reconstruction -> centroid + bounding size
rec = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
tro = rec & (r < 66)
ys, xs = np.where(tro)
cy, cx = int(ys.mean()), int(xs.mean())
h = int((ys.max() - ys.min())); w = int((xs.max() - xs.min()))
print(f"trophy centroid=({cy},{cx}) bbox h={h} w={w}", flush=True)


def avg(img, k=5, tag=""):
    return float(np.mean([b.query(img, tag=tag)[0] for _ in range(k)]))


def place(mask, size, cyy, cxx):
    """resize mask (bool) to 'size' tall keeping aspect, center at (cyy,cxx)."""
    m = Image.fromarray((mask * 255).astype("uint8"))
    ratio = size / m.height
    m = m.resize((max(1, int(m.width * ratio)), size), Image.NEAREST)
    a = np.asarray(m) > 127
    out = np.zeros((N, N), bool)
    y0 = cyy - a.shape[0] // 2; x0 = cxx - a.shape[1] // 2
    ys2, xs2 = np.where(a)
    for yy2, xx2 in zip(ys2, xs2):
        Y, X = y0 + yy2, x0 + xx2
        if 0 <= Y < N and 0 <= X < N:
            out[Y, X] = True
    return out


def load_mask(path, mode):
    im = Image.open(path).convert("RGBA")
    arr = np.asarray(im)
    if mode == "alpha":
        return arr[:, :, 3] > 40
    lum = arr[:, :, :3].mean(2)
    return (arr[:, :, 3] > 40) & (lum > 90)   # bright, opaque


results = []
for path in sorted(glob.glob(mcc.HERE + "/emoji/*.png")):
    name = os.path.basename(path)[:-4]
    for mode in ("alpha", "lumin"):
        try:
            mask = load_mask(path, mode)
        except Exception as e:
            continue
        cup = place(mask, h + 4, cy, cx)
        sc = avg(CIRCLE | cup, 5, f"em_{name}_{mode}")
        results.append((sc, name, mode))
        if b.key:
            print("KEY", b.key); raise SystemExit
results.sort(reverse=True)
print("TOP:", flush=True)
for sc, name, mode in results[:8]:
    print(f"  {sc:.3f} {name} {mode}", flush=True)
