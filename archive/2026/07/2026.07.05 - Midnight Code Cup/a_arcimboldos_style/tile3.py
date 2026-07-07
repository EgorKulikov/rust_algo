"""Tiler v3 for tests 10/11: exact-fit cells + patch-template edge libraries.

Improvements over tile2:
- cells exactly partition the dark bbox and light ring (no sliver cells)
- extra window candidates matched via FFT against actual boundary patch
  templates (average target patch per edge orientation), catching two-tone
  transition cells
- greedy tried in both draw orders (forward/reverse scanline), best kept

Usage: tile3.py DD [nd]
"""
import sys
import time

import numpy as np
from scipy.signal import fftconvolve

sys.path.insert(0, ".")
import mcc_a
from tile2 import window_library


def patch_windows(clips, patch, topn=15):
    """Best fully-opaque windows matching an exact target patch (SSD via FFT)."""
    ph, pw = patch.shape[:2]
    k = np.ones((ph, pw))
    scored = []
    for i, c in enumerate(clips):
        a = c["alpha"]
        if a.shape[0] < ph or a.shape[1] < pw:
            continue
        af = a.astype(np.float64)
        cov = fftconvolve(af, k, mode="valid")
        full = cov >= ph * pw - 0.5
        if not full.any():
            continue
        s = np.zeros_like(cov)
        for ch in range(3):
            l = c["rgb"][:, :, ch].astype(np.float64) * af
            p = patch[:, :, ch].astype(np.float64)
            s += fftconvolve(l * l, k, mode="valid")
            s -= 2 * fftconvolve(l, p[::-1, ::-1], mode="valid")
        s += (patch.astype(np.float64) ** 2).sum()
        s = np.where(full, s, np.inf)
        for _ in range(2):
            j = np.unravel_index(np.argmin(s), s.shape)
            if not np.isfinite(s[j]):
                break
            scored.append((float(s[j]), i + 1, int(j[0]), int(j[1])))
            y0 = max(j[0] - ph // 2, 0); x0 = max(j[1] - pw // 2, 0)
            s[y0 : j[0] + ph // 2, x0 : j[1] + pw // 2] = np.inf
    scored.sort()
    return [(pic, wy, wx) for _, pic, wy, wx in scored[:topn]]


def greedy(cells, libs, clips, target, order):
    h, w = target.shape[:2]
    canvas = np.full((h, w, 3), -255, dtype=np.float32)
    drawn = np.zeros((h, w), dtype=bool)
    cs = sorted(cells, key=(lambda t: (t[0], t[1])) if order == "fwd" else (lambda t: (-t[2], -t[3])))
    placements = []
    for (a0, b0, a1, b1, zone) in cs:
        lib = libs[zone]
        cell_h, cell_w = a1 - a0, b1 - b0
        best = None
        for pic, wy, wx in lib:
            clip = clips[pic - 1]
            ch, cw = clip["alpha"].shape
            if wy + cell_h > ch or wx + cell_w > cw:
                continue
            pr, pc = a0 - wy, b0 - wx
            g0, g1 = max(pr, 0), min(pr + ch, h)
            f0, f1 = max(pc, 0), min(pc + cw, w)
            if g0 >= g1 or f0 >= f1:
                continue
            al = clip["alpha"][g0 - pr : g1 - pr, f0 - pc : f1 - pc]
            rgb = clip["rgb"][g0 - pr : g1 - pr, f0 - pc : f1 - pc].astype(np.float32)
            tt = target[g0:g1, f0:f1]
            cur = canvas[g0:g1, f0:f1]
            dr = drawn[g0:g1, f0:f1]
            cellmask = np.zeros_like(dr)
            cellmask[a0 - g0 : a1 - g0, b0 - f0 : b1 - f0] = True
            count = cellmask | dr
            new = np.where(al[:, :, None], rgb, cur)
            de = (((tt - new) ** 2).sum(axis=2) * count).sum() - (((tt - cur) ** 2).sum(axis=2) * count).sum()
            if best is None or de < best[0]:
                best = (de, pic, pr, pc)
        de, pic, pr, pc = best
        placements.append((pic, pr, pc))
        clip = clips[pic - 1]
        ch, cw = clip["alpha"].shape
        g0, g1 = max(pr, 0), min(pr + ch, h)
        f0, f1 = max(pc, 0), min(pc + cw, w)
        al = clip["alpha"][g0 - pr : g1 - pr, f0 - pc : f1 - pc]
        canvas[g0:g1, f0:f1][al] = clip["rgb"][g0 - pr : g1 - pr, f0 - pc : f1 - pc][al].astype(np.float32)
        drawn[a0:a1, b0:b1] = True
    return placements


def main(dd, nd=7):
    info = mcc_a.load_input(dd)
    n = info["n"]
    target = mcc_a.load_target(info["target"]).astype(np.float32)
    clips = mcc_a.load_collection(info["collection"])
    h, w = target.shape[:2]
    dark = target.sum(axis=2) < 300
    zd = target[dark].mean(axis=0).astype(np.float64)
    zl = target[~dark].mean(axis=0).astype(np.float64)
    rows_dark = np.where(dark.mean(axis=1) > 0.5)[0]
    cols_dark = np.where(dark.mean(axis=0) > 0.5)[0]
    r0, r1 = int(rows_dark[0]), int(rows_dark[-1]) + 1
    c0, c1 = int(cols_dark[0]), int(cols_dark[-1]) + 1

    # dark cells: exact partition nd x nd
    dh = -(-(r1 - r0) // nd); dw = -(-(c1 - c0) // nd)
    dark_cells = []
    for i in range(nd):
        for j in range(nd):
            rr = r0 + i * dh; cc = c0 + j * dw
            if rr < r1 and cc < c1:
                dark_cells.append((rr, cc, min(rr + dh, r1), min(cc + dw, c1), "d"))
    n_light = n - len(dark_cells)
    area_light = h * w - (r1 - r0) * (c1 - c0)
    la = int(np.sqrt(area_light / n_light)) - 4
    light_cells = []
    def tile_rect(a0, b0, a1, b1):
        if a0 >= a1 or b0 >= b1:
            return
        ni = max(1, round((a1 - a0) / la)); nj = max(1, round((b1 - b0) / la))
        hs = (a1 - a0) / ni; ws = (b1 - b0) / nj
        for i in range(ni):
            for j in range(nj):
                light_cells.append((int(a0 + i * hs), int(b0 + j * ws),
                                    int(a0 + (i + 1) * hs), int(b0 + (j + 1) * ws), "l"))
    while True:
        light_cells.clear()
        tile_rect(0, 0, r0, w)
        tile_rect(r1, 0, h, w)
        tile_rect(r0, 0, r1, c0)
        tile_rect(r0, c1, r1, w)
        if len(dark_cells) + len(light_cells) <= n:
            break
        la += 4
    print(f"dark {len(dark_cells)} cells {dh}x{dw}, light {len(light_cells)} @ ~{la}, total {len(dark_cells)+len(light_cells)}/{n}", flush=True)

    t0 = time.time()
    cd = max(dh, dw); cl = la
    lib_d = window_library(clips, cd, zd, other_zc=zl)
    lib_l = window_library(clips, cl, zl, other_zc=zd)
    # edge patch templates: average target patch of each dark edge
    def avg_patch(cells_subset, size):
        acc = np.zeros((size, size, 3))
        cnt = 0
        for (a0, b0, a1, b1, _) in cells_subset:
            if a1 - a0 == size and b1 - b0 >= size:
                acc += target[a0:a1, b0 : b0 + size]
                cnt += 1
        return acc / max(cnt, 1) if cnt else None
    edge_sets = {
        "top": [c for c in dark_cells if c[0] == r0],
        "bottom": [c for c in dark_cells if c[2] == r1],
        "left": [c for c in dark_cells if c[1] == c0],
        "right": [c for c in dark_cells if c[3] == c1],
    }
    for name, cellset in edge_sets.items():
        if not cellset:
            continue
        size = min(cellset[0][2] - cellset[0][0], cellset[0][3] - cellset[0][1])
        p = avg_patch(cellset, size)
        if p is not None:
            lib_d += patch_windows(clips, p)
    # light inner ring templates (cells adjacent to the square)
    ring = [c for c in light_cells if (abs(c[2] - r0) < la or abs(c[0] - r1) < la or abs(c[3] - c0) < la or abs(c[1] - c1) < la)]
    if ring:
        size = min(ring[0][2] - ring[0][0], ring[0][3] - ring[0][1])
        p = avg_patch(ring, size)
        if p is not None:
            lib_l += patch_windows(clips, p)
    # dedupe
    lib_d = list(dict.fromkeys(lib_d))
    lib_l = list(dict.fromkeys(lib_l))
    print(f"libs: dark {len(lib_d)}, light {len(lib_l)} ({time.time()-t0:.0f}s)", flush=True)

    cells = dark_cells + light_cells
    libs = {"d": lib_d, "l": lib_l}
    best = None
    for order in ("fwd", "rev"):
        pl = greedy(cells, libs, clips, target, order)
        real = mcc_a.render(pl, (h, w), clips)
        err = mcc_a.score(target.astype(np.int16), real)
        print(f"order {order}: err={err:,}", flush=True)
        if best is None or err < best[0]:
            best = (err, pl)
    err, placements = best
    path = f"out/{dd:02d}g.out"
    try:
        cur = mcc_a.score_file(dd, path)
    except Exception:
        cur = None
    if cur is None or err < cur:
        with open(path, "w") as f:
            f.write(f"{len(placements)}\n")
            for pic, r, c in placements:
                f.write(f"{pic} {r} {c}\n")
        print(f"saved {path} err={err:,}")
    else:
        print(f"kept existing ({cur:,})")


if __name__ == "__main__":
    main(int(sys.argv[1]), int(sys.argv[2]) if len(sys.argv) > 2 else 7)
