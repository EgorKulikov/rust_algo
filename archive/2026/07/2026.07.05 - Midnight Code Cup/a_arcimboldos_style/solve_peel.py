"""Exact reconstruction for tests 01/03 by onion peeling.

Target = white sheet + exact letter cliparts drawn in some order. The topmost
letter matches the target on ALL its opaque pixels. Peel it (mark its pixels
wildcard), then the next letter down becomes fully matching, etc.

Weighted SSD via FFT with wildcard weights W (1=must match, 0=free):
SSD_w = corr(W*T^2, m) - 2 sum_k corr(W*T_k, m*L_k) + corr(W, m*L^2sum)
Only the W-dependent terms change per peel -> local updates.
Candidates verified exactly before peeling. Output: white + reversed peels.
"""
import sys
import time

import numpy as np
from scipy.signal import fftconvolve

sys.path.insert(0, ".")
import mcc_a

WHITE = 28


class Peeler:
    def __init__(self, dd, min_area=25):
        self.dd = dd
        info = mcc_a.load_input(dd)
        self.n = info["n"]
        self.target = mcc_a.load_target(info["target"]).astype(np.float64)
        self.clips = mcc_a.load_collection(info["collection"])
        self.h, self.w = self.target.shape[:2]
        self.min_area = min_area

        self.letters = [i for i in range(len(self.clips)) if i + 1 != WHITE]
        self.margin = max(max(self.clips[i]["alpha"].shape) for i in self.letters)
        g = self.margin
        hp, wp = self.h + 2 * g, self.w + 2 * g
        self.W = np.zeros((hp, wp))
        self.W[g : g + self.h, g : g + self.w] = 1.0
        self.T = np.zeros((hp, wp, 3))
        self.T[g : g + self.h, g : g + self.w] = self.target

        # per-letter kernels
        self.kern = {}
        for i in self.letters:
            c = self.clips[i]
            m = c["alpha"].astype(np.float64)
            l = c["rgb"].astype(np.float64) * m[:, :, None]
            l2 = (l * c["rgb"]).sum(axis=2)  # m * L^2 summed channels
            self.kern[i] = (m, l, l2)

        t0 = time.time()
        self.ssd = {}
        self.area = {}
        self.smin = {}
        for i in self.letters:
            self.compute_maps(i)
        print(f"init maps: {time.time()-t0:.0f}s", flush=True)

        self.peels = []  # top -> bottom

    def compute_maps(self, i, region=None):
        """region: (r0,r1,c0,c1) in padded pixel coords where W changed."""
        m, l, l2 = self.kern[i]
        mh, mw = m.shape
        if i not in self.ssd or region is None:
            wt2 = self.W * (self.T**2).sum(axis=2)
            s = fftconvolve(wt2, m[::-1, ::-1], mode="valid")
            for k in range(3):
                s -= 2 * fftconvolve(self.W * self.T[:, :, k], l[::-1, ::-1, k], mode="valid")
            s += fftconvolve(self.W, l2[::-1, ::-1], mode="valid")
            a = fftconvolve(self.W, m[::-1, ::-1], mode="valid")
            self.ssd[i] = s
            self.area[i] = a
        else:
            r0, r1, c0, c1 = region
            a0 = max(r0 - mh + 1, 0); b0 = max(c0 - mw + 1, 0)
            a1 = min(r1, self.ssd[i].shape[0]); b1 = min(c1, self.ssd[i].shape[1])
            if a0 >= a1 or b0 >= b1:
                return
            Ws = self.W[a0 : a1 + mh - 1, b0 : b1 + mw - 1]
            Ts = self.T[a0 : a1 + mh - 1, b0 : b1 + mw - 1]
            wt2 = Ws * (Ts**2).sum(axis=2)
            s = fftconvolve(wt2, m[::-1, ::-1], mode="valid")
            for k in range(3):
                s -= 2 * fftconvolve(Ws * Ts[:, :, k], l[::-1, ::-1, k], mode="valid")
            s += fftconvolve(Ws, l2[::-1, ::-1], mode="valid")
            a = fftconvolve(Ws, m[::-1, ::-1], mode="valid")
            self.ssd[i][a0:a1, b0:b1] = s
            self.area[i][a0:a1, b0:b1] = a

    def verify(self, i, y, x):
        """(y,x) padded map coords = clipart top-left in padded frame.
        Check all opaque non-wildcard pixels match target exactly.
        Returns exact non-wildcard matched area, or -1 if mismatch."""
        c = self.clips[i]
        m = c["alpha"]
        mh, mw = m.shape
        Wl = self.W[y : y + mh, x : x + mw]
        Tl = self.T[y : y + mh, x : x + mw]
        act = m & (Wl > 0.5)
        if not act.any():
            return 0
        diff = np.abs(Tl - c["rgb"])[act]
        if diff.max() > 0:
            return -1
        return int(act.sum())

    def peel_once(self, eps=2000.0):
        """Find best verified candidate; peel it. Returns (i,y,x,area) or None."""
        best = None  # (area, i, y, x)
        for i in self.letters:
            s = self.ssd[i]
            a = self.area[i]
            cand = (s <= eps) & (a >= (self.min_area if best is None else best[0] + 1))
            if not cand.any():
                continue
            ys, xs = np.where(cand)
            areas = a[ys, xs]
            order = np.argsort(-areas)
            for j in order[:50]:
                y, x = int(ys[j]), int(xs[j])
                va = self.verify(i, y, x)
                if va >= self.min_area and (best is None or va > best[0]):
                    best = (va, i, y, x)
                    break
        if best is None:
            return None
        va, i, y, x = best
        c = self.clips[i]
        mh, mw = c["alpha"].shape
        self.W[y : y + mh, x : x + mw][c["alpha"]] = 0.0
        self.peels.append((i + 1, y - self.margin, x - self.margin))
        region = (y, y + mh, x, x + mw)
        for j in self.letters:
            self.compute_maps(j, region)
        return best

    def run(self, max_peels=None):
        max_peels = max_peels or (self.n - 1)
        t0 = time.time()
        while len(self.peels) < max_peels:
            r = self.peel_once()
            if r is None:
                # relax min area at the tail
                if self.min_area > 1:
                    self.min_area = 1
                    r = self.peel_once()
                    if r is None:
                        break
                else:
                    break
            if len(self.peels) % 25 == 0:
                g = self.margin
                left = int((self.W[g : g + self.h, g : g + self.w] > 0.5).sum())
                print(f"peeled {len(self.peels)}, nonwild px left {left:,} ({time.time()-t0:.0f}s)", flush=True)
                self.save()
        g = self.margin
        Win = self.W[g : g + self.h, g : g + self.w] > 0.5
        rem = self.target[Win]
        nonwhite = int((np.abs(rem - 255).max(axis=1) > 0).sum()) if rem.size else 0
        print(f"done: {len(self.peels)} peels, uncovered nonwhite px: {nonwhite:,}", flush=True)
        self.save()

    def save(self):
        placements = [(WHITE, 0, 0)] + self.peels[::-1]
        mcc_a.write_out(self.dd, placements)
        return placements


if __name__ == "__main__":
    dd = int(sys.argv[1])
    p = Peeler(dd)
    p.run()
    print("final:", f"{mcc_a.score_file(dd):,}")
