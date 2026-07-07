"""Refine B: exact circle (R~118) + cleaned central cup, fine-tune, keep best."""
import numpy as np
from collections import deque
from PIL import Image
import mcc

N = mcc.N
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
cx = cy = N / 2.0
r = np.sqrt((xx - cx) ** 2 + (yy - cy) ** 2)
rec = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
G = 64


def avg(a, k=6, tag=""):
    return float(np.mean([b.query(a, tag=tag)[0] for _ in range(k)]))


def keep_large_px(mask, minsz):
    """8-connected component filter on the pixel mask (via 4px blocks)."""
    grid = mask.reshape(G, 4, G, 4).mean(axis=(1, 3)) > 0.5
    seen = np.zeros_like(grid); out = np.zeros_like(grid)
    for i in range(G):
        for j in range(G):
            if grid[i, j] and not seen[i, j]:
                q = deque([(i, j)]); seen[i, j] = True; cells = [(i, j)]
                while q:
                    y, x = q.popleft()
                    for dy in (-1, 0, 1):
                        for dx in (-1, 0, 1):
                            ny, nx = y + dy, x + dx
                            if 0 <= ny < G and 0 <= nx < G and grid[ny, nx] and not seen[ny, nx]:
                                seen[ny, nx] = True; q.append((ny, nx)); cells.append((ny, nx))
                if len(cells) >= minsz:
                    for (y, x) in cells:
                        out[y, x] = True
    return np.kron(out, np.ones((4, 4), bool))


cup_raw = rec & (r < 75)
cup_cc = keep_large_px(cup_raw, 5)
print(f"cup raw {cup_raw.sum()} cc {cup_cc.sum()}", flush=True)
best = (-1, None, "")
for R in (117, 118, 119):
    for t in (4, 5, 6):
        ring = np.abs(r - R) <= t / 2.0
        for cupname, cup in (("raw", cup_raw), ("cc", cup_cc)):
            img = ring | cup
            sc = avg(img, 6, f"rf_R{R}t{t}{cupname}")
            if sc > best[0]:
                best = (sc, img.copy(), f"R{R}t{t}cup_{cupname}")
            if b.key:
                print("KEY", b.key)
                raise SystemExit
        print(f"  R={R} t={t}: raw/cc tested, best so far {best[0]:.3f} ({best[2]})", flush=True)
print("BEST", round(best[0], 3), best[2], flush=True)
