"""Edge-correction regression around B's best image (circle+trophy).

Our best image (~95.91) differs from the target mainly at sub-block boundary
pixels. B is linear, so regress score on random FLIPS of 2px blocks in the
boundary band (symmetric): coef>0 => flipping that block helps (it's wrong in
base). Apply beneficial flips -> higher true score. base_b.npy is the fixed base.
"""
import sys
import os
import numpy as np
from PIL import Image, ImageFilter
import mcc

N = mcc.N
CB = 2
GB = N // CB
yy, xx = np.mgrid[0:N, 0:N]
r = np.sqrt((xx - N / 2) ** 2 + (yy - N / 2) ** 2)
CIRCLE = np.abs(r - 118.5) <= 2.0
rec = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127
BASE = CIRCLE | (rec & (r < 66))
np.save(mcc.HERE + "/base_b.npy", BASE)

im = Image.fromarray((BASE * 255).astype("uint8"))
band = (np.asarray(im.filter(ImageFilter.MaxFilter(9))) > 127) & \
       ~(np.asarray(im.filter(ImageFilter.MinFilter(9))) > 127)
# 2px blocks overlapping the band, symmetric left-half free variables
bandblk = band.reshape(GB, CB, GB, CB).any(axis=(1, 3))
by, bx = np.mgrid[0:GB, 0:GB]
FREE = bandblk & (bx <= GB // 2 - 1)
IDX = np.where(FREE.ravel())[0]
NR = len(IDX)
FI, FJ = IDX // GB, IDX % GB
MJ = (GB - 1) - FJ


def flip_img(bits):
    grid = np.zeros((GB, GB), bool)
    on = bits.astype(bool)
    grid[FI[on], FJ[on]] = True
    grid[FI[on], MJ[on]] = True
    flip = np.kron(grid, np.ones((CB, CB), bool))
    return BASE ^ flip


def collect(name="b", M=2500, p=0.5, seed0=90000):
    beast = mcc.Beast(name)
    path = os.path.join(mcc.HERE, f"edgedata_{name}.npz")
    masks, scores = [], []
    if os.path.exists(path):
        d = np.load(path); masks = list(d["masks"]); scores = list(d["scores"])
    print(f"[{name}] edge band free blocks={NR}, base white={BASE.mean():.4f}, resume {len(scores)}", flush=True)
    for i in range(len(scores), M):
        if beast.key:
            break
        rng = np.random.default_rng(seed0 + i)
        bits = rng.random(NR) < p
        s, _ = beast.query(flip_img(bits), tag=f"eg{i}")
        masks.append(np.packbits(bits)); scores.append(s)
        if (i + 1) % 50 == 0:
            np.savez(path, masks=np.array(masks), scores=np.array(scores))
            print(f"[{name}] {i+1}/{M}", flush=True)
    np.savez(path, masks=np.array(masks), scores=np.array(scores))
    print(f"[{name}] edge collect done {len(scores)}", flush=True)


if __name__ == "__main__":
    collect(sys.argv[1] if len(sys.argv) > 1 else "b",
            M=int(sys.argv[2]) if len(sys.argv) > 2 else 2500)
