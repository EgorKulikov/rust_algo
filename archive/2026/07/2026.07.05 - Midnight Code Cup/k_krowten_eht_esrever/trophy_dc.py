"""Drift-corrected 2px trophy regression for B.

Each sample queries (circle + random symmetric 2px trophy blocks) and the fixed
reference (circle only) BACK-TO-BACK, storing y = design - reference. The minute
/hour-scale drift is common to both and cancels, leaving only fast noise, so the
per-block signal (~0.006) is cleaner -> sharper 2px trophy that can beat the 4px
reconstruction (95.9). Reuses trophy_regress's symmetric block layout.
"""
import sys
import os
import numpy as np
import mcc
import trophy_regress as tr

N = mcc.N
CIRCLE = np.abs(np.sqrt((np.mgrid[0:N, 0:N][1] - N / 2) ** 2 +
                        (np.mgrid[0:N, 0:N][0] - N / 2) ** 2) - 118.5) <= 2.0


def collect(name="b", M=2600, p=0.5, seed0=70000):
    beast = mcc.Beast(name)
    path = os.path.join(mcc.HERE, f"trophydc_{name}.npz")
    masks, ys = [], []
    if os.path.exists(path):
        d = np.load(path); masks = list(d["masks"]); ys = list(d["ys"])
    print(f"[{name}] DC trophy: {tr.NR} free blocks, resume {len(ys)}", flush=True)
    for i in range(len(ys), M):
        if beast.key:
            break
        rng = np.random.default_rng(seed0 + i)
        bits = rng.random(tr.NR) < p
        design = tr.mask_to_img(bits)          # circle | symmetric trophy blocks
        s_d, _ = beast.query(design, tag=f"dc{i}")
        s_r, _ = beast.query(CIRCLE, tag=f"dcref{i}")   # back-to-back reference
        masks.append(np.packbits(bits)); ys.append(s_d - s_r)
        if (i + 1) % 50 == 0:
            np.savez(path, masks=np.array(masks), ys=np.array(ys))
            print(f"[{name}] {i+1}/{M} y={s_d - s_r:.3f}", flush=True)
    np.savez(path, masks=np.array(masks), ys=np.array(ys))
    print(f"[{name}] DC collect done {len(ys)}", flush=True)


if __name__ == "__main__":
    collect(sys.argv[1] if len(sys.argv) > 1 else "b",
            M=int(sys.argv[2]) if len(sys.argv) > 2 else 2600)
