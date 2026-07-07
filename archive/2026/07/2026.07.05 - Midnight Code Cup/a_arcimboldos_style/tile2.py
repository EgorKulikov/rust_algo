"""Tiler v2 for tests 10/11 (Malevich square).

- Dark bbox tiled with large cells, light ring with small cells (asymmetric
  budget), total <= N.
- Draw order: reverse scanline (bottom-right first). A tile's up-left overhang
  lands on future cells and gets buried; its down-right overhang damages only
  already-drawn cells and is charged exactly during greedy selection.
- Window library: fully-opaque windows ranked by window err + L-strip err
  (the strip of leaf content extending one cell down/right, which is what
  can leak onto drawn cells).

Usage: tile2.py DD [dark_cell] [n_dark_side]
Writes out/DDg.out.
"""
import sys
import time

import numpy as np
from scipy.signal import fftconvolve

sys.path.insert(0, ".")
import mcc_a


def window_scan(clips, cell, zc, strip_zc, transparent_pen, topn=25, strip_w=90):
    """Fully-opaque cell x cell windows ranked by window err (vs zc) plus the
    down/right L-strip cost: opaque strip pixels scored vs strip_zc,
    transparent strip pixels cost transparent_pen (0 = overhang-free edge)."""
    k = np.ones((cell, cell))
    ks = np.ones((cell + strip_w, cell + strip_w))
    scored = []
    for i, c in enumerate(clips):
        a = c["alpha"]
        if a.shape[0] < cell or a.shape[1] < cell:
            continue
        af = a.astype(np.float64)
        e = ((c["rgb"].astype(np.float64) - zc) ** 2).sum(axis=2) * af
        e_strip = ((c["rgb"].astype(np.float64) - strip_zc) ** 2).sum(axis=2) * af
        e_strip = e_strip + (1.0 - af) * transparent_pen
        cov = fftconvolve(af, k, mode="valid")
        errs = fftconvolve(e, k, mode="valid")
        # strip = UP-LEFT L-region (draw order is forward scanline: up-left
        # overhang damages drawn cells, down-right overhang gets buried).
        # Pad so the big window at core position j covers rows/cols
        # j-strip_w .. j+cell; out-of-leaf = transparent_pen.
        es_pad = np.full((a.shape[0] + strip_w, a.shape[1] + strip_w), float(transparent_pen))
        es_pad[strip_w:, strip_w:] = e_strip
        big = fftconvolve(es_pad, ks, mode="valid")
        full = cov >= cell * cell - 0.5
        if not full.any():
            continue
        errs2 = np.where(full, errs, np.inf)
        em = errs2.copy()
        for _ in range(4):
            j = np.unravel_index(np.argmin(em), em.shape)
            if not np.isfinite(em[j]):
                break
            we = errs2[j] / (cell * cell)
            strip = (big[j] - errs[j]) / ((cell + strip_w) ** 2 - cell * cell)
            se = we + 0.7 * max(float(strip), 0.0)
            scored.append((se, we, i + 1, int(j[0]), int(j[1])))
            y0 = max(j[0] - cell // 2, 0); x0 = max(j[1] - cell // 2, 0)
            em[y0 : j[0] + cell // 2, x0 : j[1] + cell // 2] = np.inf
    scored.sort()
    return [(pic, wy, wx) for _, _, pic, wy, wx in scored[:topn]]


def window_library(clips, cell, zc, other_zc=None, topn=25):
    """Union of interior, transparent-edge and boundary-facing window sets."""
    lib = window_scan(clips, cell, zc, zc, 30000.0, topn=topn)
    lib += window_scan(clips, cell, zc, zc, 0.0, topn=topn)
    if other_zc is not None:
        lib += window_scan(clips, cell, zc, other_zc, 0.0, topn=topn)
    seen = set()
    out = []
    for t in lib:
        if t not in seen:
            seen.add(t)
            out.append(t)
    return out


def main(dd, dark_cell=None, n_dark_side=None):
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
    print(f"dark bbox rows {r0}:{r1} cols {c0}:{c1}, zd={zd.round(0)}, zl={zl.round(0)}")

    nd = n_dark_side or 6
    da = max((r1 - r0 + nd - 1) // nd, (c1 - c0 + nd - 1) // nd)
    if dark_cell:
        da = dark_cell
        nd = max((r1 - r0 + da - 1) // da, (c1 - c0 + da - 1) // da)
    dark_cells = []
    for i in range(nd):
        for j in range(nd):
            rr = r0 + i * da; cc = c0 + j * da
            if rr < r1 and cc < c1:
                dark_cells.append((rr, cc, min(rr + da, r1), min(cc + da, c1), "d"))
    # light: 4 rectangles around bbox
    n_light = n - len(dark_cells)
    area_light = h * w - (r1 - r0) * (c1 - c0)
    la = int(np.sqrt(area_light / n_light))
    light_cells = []
    la -= 4  # will grow below until the layout fits the budget
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
    print(f"dark cells {len(dark_cells)} @ {da}, light cells {len(light_cells)} @ ~{la}, total {len(dark_cells)+len(light_cells)}/{n}")

    t0 = time.time()
    lib_d = window_library(clips, min(da, 260), zd, other_zc=zl)
    lib_l = window_library(clips, la, zl, other_zc=zd)
    print(f"libs: dark {len(lib_d)}, light {len(lib_l)} ({time.time()-t0:.0f}s)")

    cells = dark_cells + light_cells
    cells.sort(key=lambda t: (t[0], t[1]))  # forward scanline: stems bury down-right
    canvas = np.full((h, w, 3), -255, dtype=np.float32)
    drawn = np.zeros((h, w), dtype=bool)  # already-final area
    placements_rev = []
    t0 = time.time()
    for (a0, b0, a1, b1, zone) in cells:
        lib = lib_d if zone == "d" else lib_l
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
            # cell region counts always; drawn region counts (damage);
            # undrawn outside-cell region is neutral (buried later)
            cellmask = np.zeros_like(dr)
            cellmask[a0 - g0 : a1 - g0, b0 - f0 : b1 - f0] = True
            count = cellmask | dr
            new = np.where(al[:, :, None], rgb, cur)
            de = (((tt - new) ** 2).sum(axis=2) * count).sum() - (((tt - cur) ** 2).sum(axis=2) * count).sum()
            if best is None or de < best[0]:
                best = (de, pic, pr, pc)
        de, pic, pr, pc = best
        placements_rev.append((pic, pr, pc))
        clip = clips[pic - 1]
        ch, cw = clip["alpha"].shape
        g0, g1 = max(pr, 0), min(pr + ch, h)
        f0, f1 = max(pc, 0), min(pc + cw, w)
        al = clip["alpha"][g0 - pr : g1 - pr, f0 - pc : f1 - pc]
        canvas[g0:g1, f0:f1][al] = clip["rgb"][g0 - pr : g1 - pr, f0 - pc : f1 - pc][al].astype(np.float32)
        drawn[a0:a1, b0:b1] = True

    placements = placements_rev  # list order == draw order == z-order
    # exact score via renderer
    real = mcc_a.render(placements, (h, w), clips)
    err = mcc_a.score(target.astype(np.int16), real)
    print(f"tiled v2 err={err:,} with {len(placements)} placements ({time.time()-t0:.0f}s)")
    path = f"out/{dd:02d}g.out"
    cur_best = None
    try:
        cur_best = mcc_a.score_file(dd, path)
    except Exception:
        pass
    if cur_best is None or err < cur_best:
        with open(path, "w") as f:
            f.write(f"{len(placements)}\n")
            for pic, r, c in placements:
                f.write(f"{pic} {r} {c}\n")
        print("saved", path)
    else:
        print(f"kept existing {path} ({cur_best:,})")


if __name__ == "__main__":
    main(int(sys.argv[1]),
         int(sys.argv[2]) if len(sys.argv) > 2 else None,
         int(sys.argv[3]) if len(sys.argv) > 3 else None)
