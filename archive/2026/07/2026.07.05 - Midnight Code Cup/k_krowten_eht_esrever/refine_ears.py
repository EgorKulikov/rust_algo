"""Interleaved (drift-canceled) refinement of B's handles/ears.

The ear arc gave a confirmed +0.19. Test handle-completion candidates against the
current best using back-to-back interleaved pairs (cancels drift), keep only
improvements confirmed at >3 SE. Iterates: each accepted candidate becomes the
new base for the next round.
"""
import numpy as np
from PIL import Image
import mcc

N = mcc.N
CX = 128
b = mcc.Beast("b")
yy, xx = np.mgrid[0:N, 0:N]
cur = np.asarray(Image.open(mcc.HERE + "/best_b.png").convert("L")) > 127


def ears(cy, cx, r, t, fill=False):
    m = np.zeros((N, N), bool)
    for s in (-1, 1):
        cxs = CX + s * (cx - CX)
        rr = np.sqrt((xx - cxs) ** 2 + (yy - cy) ** 2)
        seg = (rr <= r) if fill else ((rr >= r - t) & (rr <= r))
        seg &= (xx >= cxs) if s > 0 else (xx <= cxs)
        m |= seg
    return m


def compare(cand, k=15):
    """interleaved base(cur) vs cand -> (mean delta, se)."""
    d = []
    for _ in range(k):
        s1 = b.query(cur, tag="r_base")[0]
        s2 = b.query(cand, tag="r_cand")[0]
        d.append(s2 - s1)
    d = np.array(d)
    return d.mean(), d.std() / np.sqrt(len(d))


def candidates(base):
    c = {}
    # larger / thicker outer arcs
    for r in (14, 16, 18):
        for t in (5, 7, 9):
            c[f"arc_r{r}_t{t}"] = base | ears(92, 175, r, t)
    # taller/shifted centers
    for cy in (88, 90, 94):
        c[f"arc_cy{cy}"] = base | ears(cy, 175, 15, 6)
    # filled handle
    c["fill_r15"] = base | ears(92, 175, 15, 0, fill=True)
    c["fill_r13"] = base | ears(92, 174, 13, 0, fill=True)
    return c


for rnd in range(6):
    if b.key:
        break
    best = (0.0, None, "")
    for name, cand in candidates(cur).items():
        if np.array_equal(cand, cur):
            continue
        m, se = compare(cand, 12)
        flag = "*" if (m > 3 * se and m > 0) else " "
        print(f"  {flag} {name}: {m:+.4f} +/- {se:.4f}", flush=True)
        if m > best[0] and m > 3 * se:
            best = (m, cand, name)
        if b.key:
            print("KEY", b.key)
            break
    if best[1] is None:
        print(f"round {rnd}: no confirmed improvement, stop", flush=True)
        break
    cur = best[1]
    Image.fromarray((cur * 255).astype("uint8")).save(mcc.HERE + "/best_b.png")
    print(f"round {rnd}: accepted {best[2]} (+{best[0]:.3f}), saved", flush=True)
