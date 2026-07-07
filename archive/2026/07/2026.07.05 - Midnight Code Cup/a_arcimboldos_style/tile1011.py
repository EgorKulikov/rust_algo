"""Tests 10/11 (Malevich square): tile the canvas with the best fully-opaque
sub-windows of cliparts. Placement k for cell (i,j) puts clip so its chosen
window lands exactly on the cell; scanline draw order means each tile's
down-right overhang is overwritten by later tiles, and its up-left overhang
(damage to previous cells) is accounted exactly during greedy selection.

Usage: tile1011.py DD [cell]
Writes out/DDg.out (watchdog submits if better).
"""
import sys
import time

import numpy as np
from scipy.signal import fftconvolve

sys.path.insert(0, ".")
import mcc_a


def build_library(clips, target, cell, zone_colors, per_zone=40):
    """Return list of (clip_id1b, wy, wx) candidate windows (fully opaque)."""
    lib = []
    k = np.ones((cell, cell))
    for zc in zone_colors:
        scored = []
        for i, c in enumerate(clips):
            a = c["alpha"]
            if a.shape[0] < cell or a.shape[1] < cell:
                continue
            af = a.astype(np.float64)
            e = ((c["rgb"].astype(np.float64) - zc) ** 2).sum(axis=2) * af
            cov = fftconvolve(af, k, mode="valid")
            errs = fftconvolve(e, k, mode="valid")
            full = cov >= cell * cell - 0.5
            if not full.any():
                continue
            errs[~full] = np.inf
            # a few local minima per clip, min separation cell//2
            em = errs.copy()
            for _ in range(3):
                j = np.unravel_index(np.argmin(em), em.shape)
                if not np.isfinite(em[j]):
                    break
                scored.append((float(errs[j]), i + 1, int(j[0]), int(j[1])))
                y0 = max(j[0] - cell // 2, 0); x0 = max(j[1] - cell // 2, 0)
                em[y0 : j[0] + cell // 2, x0 : j[1] + cell // 2] = np.inf
        scored.sort()
        lib.extend(scored[:per_zone])
    # dedupe
    seen = set()
    out = []
    for e, i, y, x in lib:
        if (i, y, x) in seen:
            continue
        seen.add((i, y, x))
        out.append((i, y, x))
    return out


def main(dd, cell=None):
    info = mcc_a.load_input(dd)
    n = info["n"]
    target = mcc_a.load_target(info["target"]).astype(np.float32)
    clips = mcc_a.load_collection(info["collection"])
    h, w = target.shape[:2]
    if cell is None:
        import math
        cell = int(math.ceil(max(h, w) / math.floor(math.sqrt(n))))
    rows = (h + cell - 1) // cell
    cols = (w + cell - 1) // cell
    assert rows * cols <= n, (rows, cols, n)
    print(f"cell={cell}, grid {rows}x{cols} = {rows*cols} <= {n}")

    # zones: sample distinct target colors (inner + border) via coarse kmeans-ish
    pts = target[::17, ::13].reshape(-1, 3)
    zones = [pts.mean(axis=0)]
    for _ in range(3):
        d = ((pts[:, None, :] - np.array(zones)[None]) ** 2).sum(axis=2).min(axis=1)
        zones.append(pts[np.argmax(d)])
    zones = [z for z in zones]
    print("zone colors:", [np.round(z).astype(int) for z in zones])

    t0 = time.time()
    lib = build_library(clips, target, cell, zones)
    print(f"library: {len(lib)} windows ({time.time()-t0:.0f}s)")

    canvas = np.full((h, w, 3), -255, dtype=np.float32)
    placements = []
    t0 = time.time()
    for i in range(rows):
        for j in range(cols):
            r0, c0 = i * cell, j * cell
            r1, c1 = min(r0 + cell, h), min(c0 + cell, w)
            best = None
            for pic, wy, wx in lib:
                clip = clips[pic - 1]
                ch, cw = clip["alpha"].shape
                pr, pc = r0 - wy, c0 - wx  # clip top-left on canvas
                a0, a1 = max(pr, 0), min(pr + ch, h)
                b0, b1 = max(pc, 0), min(pc + cw, w)
                if a0 >= a1 or b0 >= b1:
                    continue
                # affected region: only up to current cell's bottom-right
                # (down/right overhang will be overwritten by later tiles,
                # approximate its final effect as neutral)
                a1e = min(a1, r1); b1e = min(b1, c1)
                if a0 >= a1e or b0 >= b1e:
                    continue
                al = clip["alpha"][a0 - pr : a1e - pr, b0 - pc : b1e - pc]
                rgb = clip["rgb"][a0 - pr : a1e - pr, b0 - pc : b1e - pc].astype(np.float32)
                tt = target[a0:a1e, b0:b1e]
                cur = canvas[a0:a1e, b0:b1e]
                e_new = np.where(al[:, :, None].repeat(3, 2), rgb, cur)
                de = ((tt - e_new) ** 2).sum() - ((tt - cur) ** 2).sum()
                if best is None or de < best[0]:
                    best = (de, pic, pr, pc)
            _, pic, pr, pc = best
            placements.append((pic, pr, pc))
            clip = clips[pic - 1]
            ch, cw = clip["alpha"].shape
            a0, a1 = max(pr, 0), min(pr + ch, h)
            b0, b1 = max(pc, 0), min(pc + cw, w)
            al = clip["alpha"][a0 - pr : a1 - pr, b0 - pc : b1 - pc]
            canvas[a0:a1, b0:b1][al] = clip["rgb"][a0 - pr : a1 - pr, b0 - pc : b1 - pc][al].astype(np.float32)
        print(f"row {i+1}/{rows} err={((target-canvas)**2).sum():,.0f} ({time.time()-t0:.0f}s)", flush=True)

    err = mcc_a.score(target.astype(np.int16), canvas.astype(np.int16))
    print(f"tiled err={err:,} with {len(placements)} placements")
    path = f"out/{dd:02d}g.out"
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    print("saved", path)


if __name__ == "__main__":
    main(int(sys.argv[1]), int(sys.argv[2]) if len(sys.argv) > 2 else None)
