"""Focused 2px regression on B's trophy region (circle fixed).

The circle is nailed (R=118,t=4); the remaining error is the trophy's curved
edges/handles, which 4px blocks approximate poorly. Reconstruct the trophy at
2px: each query = clean circle + a random subset of 2px blocks (centers r<RMAX)
white, on black. Regress score on the 2px-block indicators -> exact trophy signs
(averaging beats B's noise). Collects trophydata_b.npz (resumable).
"""
import sys
import os
import numpy as np
import mcc

N = mcc.N
CB = 2                      # 2px blocks
GB = N // CB               # 128
yy, xx = np.mgrid[0:N, 0:N]
r_px = np.sqrt((xx - N / 2) ** 2 + (yy - N / 2) ** 2)
CIRCLE = np.abs(r_px - 118) <= 2.0
RMAX = 60
# block centers
by, bx = np.mgrid[0:GB, 0:GB]
br = np.sqrt(((bx + 0.5) * CB - N / 2) ** 2 + ((by + 0.5) * CB - N / 2) ** 2)
REGION = (br < RMAX)
# symmetric design: free variables are left-half region blocks (col<=63);
# each free block paints itself AND its mirror (col 127-j). Halves unknowns.
FREE = REGION & (bx <= GB // 2 - 1)
IDX = np.where(FREE.ravel())[0]     # free (left-half) block indices
NR = len(IDX)
_fi = IDX // GB
_fj = IDX % GB
_mj = (GB - 1) - _fj                # mirror columns


def mask_to_img(bits_region):
    grid = np.zeros((GB, GB), bool)
    on = bits_region.astype(bool)
    grid[_fi[on], _fj[on]] = True
    grid[_fi[on], _mj[on]] = True    # mirror
    trophy = np.kron(grid, np.ones((CB, CB), bool))
    return CIRCLE | trophy


def collect(name="b", M=7000, p=0.5, seed0=50000):
    beast = mcc.Beast(name)
    path = os.path.join(mcc.HERE, f"trophydata_{name}.npz")
    masks, scores = [], []
    if os.path.exists(path):
        d = np.load(path)
        masks = list(d["masks"]); scores = list(d["scores"])
    print(f"[{name}] trophy 2px region blocks={NR}, resume {len(scores)}", flush=True)
    for i in range(len(scores), M):
        if beast.key:
            break
        rng = np.random.default_rng(seed0 + i)
        bits = rng.random(NR) < p
        s, _ = beast.query(mask_to_img(bits), tag=f"tr{i}")
        masks.append(np.packbits(bits)); scores.append(s)
        if (i + 1) % 50 == 0:
            np.savez(path, masks=np.array(masks), scores=np.array(scores))
            print(f"[{name}] {i+1}/{M}", flush=True)
    np.savez(path, masks=np.array(masks), scores=np.array(scores))
    print(f"[{name}] trophy collect done {len(scores)}", flush=True)


if __name__ == "__main__":
    collect(sys.argv[1] if len(sys.argv) > 1 else "b",
            M=int(sys.argv[2]) if len(sys.argv) > 2 else 7000)
