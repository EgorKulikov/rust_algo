"""Tests 07-12: collage with leaves/flowers/vegetables via generic greedy
savings placement (same math as solve_letters, no white sheet, float32 maps),
plus a replace-refinement pass.
"""
import sys
import time

import numpy as np
from scipy.signal import fftconvolve

sys.path.insert(0, ".")
import mcc_a
from solve_letters import block_mean


class CollageGreedy:
    def __init__(self, dd, d=8, budget=None, resume=False):
        self.dd = dd
        self.d = d
        info = mcc_a.load_input(dd)
        self.n = min(info["n"], budget or info["n"])
        self.target = mcc_a.load_target(info["target"])
        self.clips = mcc_a.load_collection(info["collection"])
        self.h, self.w = self.target.shape[:2]
        self.canvas = np.full((self.h, self.w, 3), -255, dtype=np.int16)
        self.placements = []

        self.tds = block_mean(self.target.astype(np.float64), d)
        self.lds = []
        self.margin = 0
        for c in self.clips:
            m = block_mean(c["alpha"].astype(np.float64), d)
            rgb = c["rgb"] * c["alpha"][:, :, None]
            r = block_mean(rgb.astype(np.float64), d)
            self.lds.append((m, r))
            self.margin = max(self.margin, max(m.shape))
        hh, ww = self.tds.shape[:2]
        g = self.margin
        self.wpad = np.zeros((hh + 2 * g, ww + 2 * g))
        self.wpad[g : g + hh, g : g + ww] = 1.0
        self.tpad = np.zeros((hh + 2 * g, ww + 2 * g, 3))
        self.tpad[g : g + hh, g : g + ww] = self.tds

        t0 = time.time()
        self.static = []
        wt2 = self.wpad * (self.tpad**2).sum(axis=2)
        for m, r in self.lds:
            with np.errstate(invalid="ignore", divide="ignore"):
                lmean = np.where(m[:, :, None] > 1e-9, r / np.maximum(m, 1e-9)[:, :, None], 0.0)
            ml = m[:, :, None] * lmean
            ml2 = (m[:, :, None] * lmean**2).sum(axis=2)
            s = fftconvolve(wt2, m[::-1, ::-1], mode="valid")
            for k in range(3):
                s -= 2 * fftconvolve(self.wpad * self.tpad[:, :, k], ml[::-1, ::-1, k], mode="valid")
            s += fftconvolve(self.wpad, ml2[::-1, ::-1], mode="valid")
            self.static.append(s.astype(np.float32))
        print(f"static maps: {time.time()-t0:.0f}s, margin={g}")

        if resume:
            import os
            path = resume if isinstance(resume, str) else os.path.join(mcc_a.OUT, f"{dd:02d}.out")
            for pic, r, c in mcc_a.read_out(path):
                self.place(pic, r, c)
            print(f"resumed {len(self.placements)} placements")

        self.emap = None
        self.savings = [None] * len(self.clips)
        self.smax = np.full(len(self.clips), -np.inf)
        self.recompute_emap()
        for i in range(len(self.clips)):
            self.update_savings(i)
        print(f"init done: {time.time()-t0:.0f}s")

    def recompute_emap(self, region=None):
        d = self.d
        if self.emap is None:
            self.emap = np.zeros_like(self.wpad)
            region = None
        g = self.margin
        if region is None:
            err = (self.target.astype(np.float64) - self.canvas) ** 2
            eds = block_mean(err.sum(axis=2), d)
            self.emap[g : g + eds.shape[0], g : g + eds.shape[1]] = eds
        else:
            r0, r1, c0, c1 = region  # ds coords
            fr0, fr1 = r0 * d, min(r1 * d, self.h)
            fc0, fc1 = c0 * d, min(c1 * d, self.w)
            err = (self.target[fr0:fr1, fc0:fc1].astype(np.float64) - self.canvas[fr0:fr1, fc0:fc1]) ** 2
            eds = block_mean(err.sum(axis=2), d)
            self.emap[g + r0 : g + r0 + eds.shape[0], g + c0 : g + c0 + eds.shape[1]] = eds

    def update_savings(self, i, region=None):
        m, _ = self.lds[i]
        if self.savings[i] is None or region is None:
            e = fftconvolve(self.emap, m[::-1, ::-1], mode="valid")
            self.savings[i] = (e - self.static[i]).astype(np.float32)
        else:
            r0, r1, c0, c1 = region
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
            return -1.0
        a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
        t = self.target[r0:r1, c0:c1].astype(np.float32)
        cur = self.canvas[r0:r1, c0:c1].astype(np.float32)
        l = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c].astype(np.float32)
        e_cur = ((t - cur) ** 2).sum(axis=2)
        e_new = ((t - l) ** 2).sum(axis=2)
        return float(((e_cur - e_new) * a).sum())

    def step(self, refine=None, refine_step=2):
        refine = refine if refine is not None else self.d // 2
        i = int(np.argmax(self.smax))
        if self.smax[i] <= 0:
            return None
        s = self.savings[i]
        y, x = np.unravel_index(np.argmax(s), s.shape)
        d, g = self.d, self.margin
        r0 = (y - g) * d
        c0 = (x - g) * d
        best = (-1.0, r0, c0)
        for dr in range(-refine, refine + d, refine_step):
            for dc in range(-refine, refine + d, refine_step):
                v = self.exact_savings(i + 1, r0 + dr, c0 + dc)
                if v > best[0]:
                    best = (v, r0 + dr, c0 + dc)
        v, r, c = best
        if v <= 0:
            # kill a neighborhood: coarse maps overestimate, avoid livelock;
            # scale with kernel so huge cliparts don't need thousands of kills
            m = self.lds[i][0]
            K = max(3, max(m.shape) // 6)
            self.savings[i][max(y - K, 0) : y + K + 1, max(x - K, 0) : x + K + 1] = -np.inf
            self.smax[i] = self.savings[i].max()
            return 0.0
        rr = self.place(i + 1, r, c)
        r0, r1, c0, c1 = rr
        dr0, dr1 = r0 // d, (r1 + d - 1) // d
        dc0, dc1 = c0 // d, (c1 + d - 1) // d
        self.recompute_emap((dr0, dr1, dc0, dc1))
        region = (g + dr0, g + dr1, g + dc0, g + dc1)
        for j in range(len(self.clips)):
            self.update_savings(j, region)
        return v

    def run(self, log_every=25, refine_step=2):
        t0 = time.time()
        placed = len(self.placements)
        kills = 0
        while placed < self.n:
            v = self.step(refine_step=refine_step)
            if v is None:
                print(f"converged after {placed}", flush=True)
                break
            if v > 0:
                placed += 1
                kills = 0
                if placed % log_every == 0:
                    err = mcc_a.score(self.target, self.canvas)
                    print(f"{placed}/{self.n} err={err:,} kills_seen ({time.time()-t0:.0f}s)", flush=True)
                    mcc_a.write_out(self.dd, self.placements)
            else:
                kills += 1
                if kills % 500 == 0:
                    print(f"{kills} consecutive kills at placed={placed} ({time.time()-t0:.0f}s)", flush=True)
                if kills >= 5000:
                    print("kill limit reached, stopping greedy", flush=True)
                    break
        return self.placements

    def save(self):
        mcc_a.write_out(self.dd, self.placements)
        err = mcc_a.score(self.target, self.canvas)
        print(f"saved {self.dd}: {len(self.placements)} placements, err={err:,}")
        return err


if __name__ == "__main__":
    dd = int(sys.argv[1])
    d = int(sys.argv[2]) if len(sys.argv) > 2 else 8
    budget = int(sys.argv[3]) if len(sys.argv) > 3 else None
    g = CollageGreedy(dd, d=d, budget=budget, resume="--resume" in sys.argv)
    g.run(refine_step=int(sys.argv[4]) if len(sys.argv) > 4 and sys.argv[4].isdigit() else 2)
    g.save()
