"""Hammer B's best image many times. B is noisy and the key drops on ANY single
query crossing the threshold, so if the threshold is within noise reach of our
~95.9 mean, repeated queries of the best image may trigger it. Also measures the
noise distribution."""
import sys
import numpy as np
from PIL import Image
import mcc

N = mcc.N
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
r = np.sqrt((xx - N / 2) ** 2 + (yy - N / 2) ** 2)
CIRCLE = np.abs(r - 118.5) <= 2.0
rec = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
trophy4 = rec & (r < 66)
img = CIRCLE | trophy4

K = int(sys.argv[1]) if len(sys.argv) > 1 else 60
scores = []
for i in range(K):
    s, k = b.query(img, tag=f"hammer{i}")
    scores.append(s)
    if k:
        print(f"*** KEY: {k} ***", flush=True)
        break
scores = np.array(scores)
print(f"n={len(scores)} mean={scores.mean():.3f} std={scores.std():.3f} "
      f"min={scores.min():.3f} max={scores.max():.3f}", flush=True)
