"""Greedy letter placement for tests 01-03.

Strategy: clipart 28 (giant opaque white sheet) covers the whole canvas as
the bottom layer. Then greedily add letters on top, each step choosing the
(letter, position) with max savings = err(current) - err(letter drawn there).

Savings maps are kept per letter at a downscale factor D via FFT correlation;
only the error-map term depends on the canvas, so after each placement just
that term is updated locally. Chosen candidates are refined at full res by
trying small shifts and measuring exact savings.
"""
import sys
import time

import numpy as np
from scipy.signal import fftconvolve

sys.path.insert(0, ".")
import mcc_a

WHITE = 28  # letters collection: 2000x5400 all-white opaque sheet


def block_mean(a, d):
    h, w = a.shape[:2]
    if h < d or w < d:  # tiny clipart: collapse to a single cell
        if a.ndim == 3:
            return a.mean(axis=(0, 1)).reshape(1, 1, a.shape[2])
        return a.mean().reshape(1, 1)
    hh, ww = h // d * d, w // d * d
    a = a[:hh, :ww]
    if a.ndim == 3:
        return a.reshape(hh // d, d, ww // d, d, a.shape[2]).mean(axis=(1, 3))
    return a.reshape(hh // d, d, ww // d, d).mean(axis=(1, 3))


class LetterGreedy:
    def __init__(self, dd, d=4, use_white=True, resume_path=None):
        self.dd = dd
        self.d = d
        info = mcc_a.load_input(dd)
        self.n = info["n"]
        self.target = mcc_a.load_target(info["target"])
        self.clips = mcc_a.load_collection(info["collection"])
        self.h, self.w = self.target.shape[:2]
        self.canvas = np.full((self.h, self.w, 3), -255, dtype=np.int16)
        self.placements = []
        if resume_path:
            for pic, r, c in mcc_a.read_out(resume_path):
                self.place(pic, r, c)
            print(f"resumed {len(self.placements)} placements", flush=True)
        elif use_white:
            self.place(WHITE, 0, 0)

        # downscaled data, padded by margin so letters can hang off edges
        self.tds = block_mean(self.target.astype(np.float64), d)
        self.lds = []  # (mask_frac, rgb_mean, kern_const) per letter
        self.margin = 0
        for i, c in enumerate(self.clips):
            if i + 1 == WHITE:
                self.lds.append(None)
                continue
            m = block_mean(c["alpha"].astype(np.float64), d)
            rgb = c["rgb"] * c["alpha"][:, :, None]
            r = block_mean(rgb.astype(np.float64), d)  # premultiplied mean
            self.lds.append((m, r))
            self.margin = max(self.margin, max(m.shape))
        hh, ww = self.tds.shape[:2]
        g = self.margin
        self.wpad = np.zeros((hh + 2 * g, ww + 2 * g))
        self.wpad[g : g + hh, g : g + ww] = 1.0
        self.tpad = np.zeros((hh + 2 * g, ww + 2 * g, 3))
        self.tpad[g : g + hh, g : g + ww] = self.tds

        # static term per letter: sum over mask of (T-L)^2, weighted by wpad
        # = corr(w*T^2sum, m) - 2 sum_k corr(w*T_k, m*L_k) + corr(w, m*L^2sum)
        self.static = []
        wt2 = self.wpad * (self.tpad**2).sum(axis=2)
        for i, l in enumerate(self.lds):
            if l is None:
                self.static.append(None)
                continue
            m, r = l
            with np.errstate(invalid="ignore", divide="ignore"):
                lmean = np.where(m[:, :, None] > 1e-9, r / np.maximum(m, 1e-9)[:, :, None], 0.0)
            ml = m[:, :, None] * lmean
            ml2 = (m[:, :, None] * lmean**2).sum(axis=2)
            s = fftconvolve(wt2, m[::-1, ::-1], mode="valid")
            for k in range(3):
                s -= 2 * fftconvolve(self.wpad * self.tpad[:, :, k], ml[::-1, ::-1, k], mode="valid")
            s += fftconvolve(self.wpad, ml2[::-1, ::-1], mode="valid")
            self.static.append(s)

        self.emap = None  # padded downscaled error map
        self.savings = [None] * len(self.clips)
        self.smax = np.full(len(self.clips), -np.inf)
        self.recompute_emap()
        for i in range(len(self.clips)):
            self.update_savings(i)

    def recompute_emap(self, region=None):
        """region: (r0,r1,c0,c1) in downscale coords, else full."""
        d = self.d
        if self.emap is None:
            self.emap = np.zeros_like(self.wpad)
            region = None
        err = (self.target.astype(np.float64) - self.canvas) ** 2
        eds = block_mean(err.sum(axis=2), d)
        g = self.margin
        if region is None:
            self.emap[g : g + eds.shape[0], g : g + eds.shape[1]] = eds
        else:
            r0, r1, c0, c1 = region
            r0 = max(r0, 0); c0 = max(c0, 0)
            r1 = min(r1, eds.shape[0]); c1 = min(c1, eds.shape[1])
            self.emap[g + r0 : g + r1, g + c0 : g + c1] = eds[r0:r1, c0:c1]

    def update_savings(self, i, region=None):
        """savings_i = corr(emap, m_i) - static_i (scaled by d^2 implicitly)."""
        if self.lds[i] is None:
            return
        m, _ = self.lds[i]
        if self.savings[i] is None or region is None:
            e = fftconvolve(self.emap, m[::-1, ::-1], mode="valid")
            self.savings[i] = e - self.static[i]
        else:
            r0, r1, c0, c1 = region  # changed emap rows (padded coords)
            mh, mw = m.shape
            a0 = max(r0 - mh + 1, 0); b0 = max(c0 - mw + 1, 0)
            a1 = min(r1, self.savings[i].shape[0]); b1 = min(c1, self.savings[i].shape[1])
            if a0 >= a1 or b0 >= b1:
                return
            sub = self.emap[a0 : a1 + mh - 1, b0 : b1 + mw - 1]
            e = fftconvolve(sub, m[::-1, ::-1], mode="valid")
            self.savings[i][a0:a1, b0:b1] = e - self.static[i][a0:a1, b0:b1]
        self.smax[i] = self.savings[i].max()

    def place(self, pic, r, c):
        self.placements.append((pic, r, c))
        clip = self.clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, self.h)
        c0, c1 = max(c, 0), min(c + cw, self.w)
        if r0 < r1 and c0 < c1:
            a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
            self.canvas[r0:r1, c0:c1][a] = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c][a]
        return (r0, r1, c0, c1)

    def exact_savings(self, pic, r, c):
        clip = self.clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, self.h)
        c0, c1 = max(c, 0), min(c + cw, self.w)
        if r0 >= r1 or c0 >= c1:
            return -1
        a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
        t = self.target[r0:r1, c0:c1].astype(np.float64)
        cur = self.canvas[r0:r1, c0:c1].astype(np.float64)
        l = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c].astype(np.float64)
        e_cur = ((t - cur) ** 2).sum(axis=2)
        e_new = ((t - l) ** 2).sum(axis=2)
        return float(((e_cur - e_new) * a).sum())

    def step(self, refine=3):
        """One greedy placement. Returns savings or None if no positive move."""
        i = int(np.argmax(self.smax))
        if self.smax[i] <= 0:
            return None
        s = self.savings[i]
        y, x = np.unravel_index(np.argmax(s), s.shape)
        d, g = self.d, self.margin
        # full-res position estimate
        r0 = (y - g) * d
        c0 = (x - g) * d
        best = (-1, r0, c0)
        for dr in range(-refine, refine + d):
            for dc in range(-refine, refine + d):
                v = self.exact_savings(i + 1, r0 + dr, c0 + dc)
                if v > best[0]:
                    best = (v, r0 + dr, c0 + dc)
        v, r, c = best
        if v <= 0:
            # kill this cell to avoid livelock, retry next step
            self.savings[i][y, x] = -np.inf
            self.smax[i] = self.savings[i].max()
            return 0
        reg = self.place(i + 1, r, c)
        r0, r1, c0, c1 = reg
        dr0, dr1 = r0 // d, (r1 + d - 1) // d
        dc0, dc1 = c0 // d, (c1 + d - 1) // d
        self.recompute_emap((dr0, dr1, dc0, dc1))
        region = (g + dr0, g + dr1, g + dc0, g + dc1)
        for j in range(len(self.clips)):
            self.update_savings(j, region)
        return v

    def run(self, budget=None, log_every=25):
        budget = min(budget or self.n, self.n) - len(self.placements)
        t0 = time.time()
        placed = 0
        while placed < budget:
            v = self.step()
            if v is None:
                print(f"no positive savings left after {placed}", flush=True)
                break
            if v > 0:
                placed += 1
                if placed % log_every == 0:
                    err = mcc_a.score(self.target, self.canvas)
                    print(f"{placed}/{budget} err={err:,} ({time.time()-t0:.0f}s)", flush=True)
                    mcc_a.write_out(self.dd, self.placements)
        return self.placements


if __name__ == "__main__":
    dd = int(sys.argv[1])
    d = int(sys.argv[2]) if len(sys.argv) > 2 else 4
    g = LetterGreedy(dd, d=d)
    g.run()
    path = mcc_a.write_out(dd, g.placements)
    print("final:", f"{mcc_a.score_file(dd):,}", "placements:", len(g.placements))
