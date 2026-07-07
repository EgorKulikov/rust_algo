"""Fast onion peeling (v2) for tests 01/03.

Improvements over solve_peel:
- candidate cache: full scan once, then only rescan the window touched by
  each peel (SSD is monotone nonincreasing as wildcards grow, so candidates
  only ever appear; stale ones get re-checked on pop).
- colored gate: only peel placements matching >=1 non-white target pixel
  (white-on-white matches are redundant w.r.t. the white sheet and explode).
- peel priority: most colored matched pixels first.
"""
import heapq
import sys
import time

import numpy as np
from scipy.signal import fftconvolve

sys.path.insert(0, ".")
import mcc_a

WHITE = 28
EPS = 2000.0


class Peeler2:
    def __init__(self, dd):
        self.dd = dd
        info = mcc_a.load_input(dd)
        self.n = info["n"]
        self.target = mcc_a.load_target(info["target"]).astype(np.float64)
        self.clips = mcc_a.load_collection(info["collection"])
        self.h, self.w = self.target.shape[:2]

        self.letters = [i for i in range(len(self.clips)) if i + 1 != WHITE]
        self.margin = max(max(self.clips[i]["alpha"].shape) for i in self.letters)
        g = self.margin
        hp, wp = self.h + 2 * g, self.w + 2 * g
        self.W = np.zeros((hp, wp))
        self.W[g : g + self.h, g : g + self.w] = 1.0
        self.T = np.zeros((hp, wp, 3))
        self.T[g : g + self.h, g : g + self.w] = self.target
        colored = (np.abs(self.T - 255).max(axis=2) > 0) & (self.W > 0.5)
        self.Wc = colored.astype(np.float64)  # non-white, non-wildcard

        self.kern = {}
        for i in self.letters:
            c = self.clips[i]
            m = c["alpha"].astype(np.float64)
            l = c["rgb"].astype(np.float64) * m[:, :, None]
            l2 = (l * c["rgb"]).sum(axis=2)
            self.kern[i] = (m, l, l2)

        t0 = time.time()
        self.ssd = {}
        self.ac = {}  # colored matched area
        for i in self.letters:
            self.compute_maps(i)
        print(f"init maps: {time.time()-t0:.0f}s", flush=True)

        self.heap = []  # (-colored_area, i, y, x)
        self.pushed = {}
        for i in self.letters:
            self.scan(i)
        print(f"init candidates: {len(self.heap)}", flush=True)
        self.peels = []

    def compute_maps(self, i, region=None):
        m, l, l2 = self.kern[i]
        mh, mw = m.shape
        if i not in self.ssd or region is None:
            wt2 = self.W * (self.T**2).sum(axis=2)
            s = fftconvolve(wt2, m[::-1, ::-1], mode="valid")
            for k in range(3):
                s -= 2 * fftconvolve(self.W * self.T[:, :, k], l[::-1, ::-1, k], mode="valid")
            s += fftconvolve(self.W, l2[::-1, ::-1], mode="valid")
            self.ssd[i] = s
            self.ac[i] = fftconvolve(self.Wc, m[::-1, ::-1], mode="valid")
        else:
            r0, r1, c0, c1 = region
            a0 = max(r0 - mh + 1, 0); b0 = max(c0 - mw + 1, 0)
            a1 = min(r1, self.ssd[i].shape[0]); b1 = min(c1, self.ssd[i].shape[1])
            if a0 >= a1 or b0 >= b1:
                return None
            Ws = self.W[a0 : a1 + mh - 1, b0 : b1 + mw - 1]
            Ts = self.T[a0 : a1 + mh - 1, b0 : b1 + mw - 1]
            wt2 = Ws * (Ts**2).sum(axis=2)
            s = fftconvolve(wt2, m[::-1, ::-1], mode="valid")
            for k in range(3):
                s -= 2 * fftconvolve(Ws * Ts[:, :, k], l[::-1, ::-1, k], mode="valid")
            s += fftconvolve(Ws, l2[::-1, ::-1], mode="valid")
            self.ssd[i][a0:a1, b0:b1] = s
            acs = fftconvolve(self.Wc[a0 : a1 + mh - 1, b0 : b1 + mw - 1], m[::-1, ::-1], mode="valid")
            self.ac[i][a0:a1, b0:b1] = acs
            return (a0, a1, b0, b1)

    def scan(self, i, box=None):
        s = self.ssd[i]
        a = self.ac[i]
        if box is None:
            sub_s, sub_a, off = s, a, (0, 0)
        else:
            a0, a1, b0, b1 = box
            sub_s, sub_a, off = s[a0:a1, b0:b1], a[a0:a1, b0:b1], (a0, b0)
        ys, xs = np.where((sub_s <= EPS) & (sub_a >= 0.5))
        for y, x in zip(ys.tolist(), xs.tolist()):
            key = (i, y + off[0], x + off[1])
            ac = sub_a[y, x]
            prev = self.pushed.get(key)
            if prev is not None and abs(prev - ac) < 2.0:
                continue
            self.pushed[key] = ac
            heapq.heappush(self.heap, (-ac, i, y + off[0], x + off[1]))

    def verify(self, i, y, x):
        c = self.clips[i]
        m = c["alpha"]
        mh, mw = m.shape
        Wl = self.W[y : y + mh, x : x + mw]
        Tl = self.T[y : y + mh, x : x + mw]
        act = m & (Wl > 0.5)
        if not act.any():
            return -1
        if np.abs(Tl - c["rgb"])[act].max() > 0:
            return -1
        return int(act.sum())

    def run(self, max_peels=None, log_every=25):
        max_peels = max_peels or (self.n - 1)
        t0 = time.time()
        while self.heap and len(self.peels) < max_peels:
            negac, i, y, x = heapq.heappop(self.heap)
            # stale checks
            if self.ssd[i][y, x] > EPS or self.ac[i][y, x] < 0.5:
                continue
            if -negac > self.ac[i][y, x] + 0.5:  # area shrank; reinsert with current
                heapq.heappush(self.heap, (-self.ac[i][y, x], i, y, x))
                continue
            if self.verify(i, y, x) < 0:
                continue
            c = self.clips[i]
            mh, mw = c["alpha"].shape
            self.W[y : y + mh, x : x + mw][c["alpha"]] = 0.0
            self.Wc[y : y + mh, x : x + mw][c["alpha"]] = 0.0
            self.peels.append((i + 1, y - self.margin, x - self.margin))
            region = (y, y + mh, x, x + mw)
            for j in self.letters:
                box = self.compute_maps(j, region)
                if box is not None:
                    self.scan(j, box)
            if len(self.peels) % log_every == 0:
                g = self.margin
                left = int(self.Wc[g : g + self.h, g : g + self.w].sum())
                print(f"peeled {len(self.peels)}, colored px left {left:,}, heap {len(self.heap)} ({time.time()-t0:.0f}s)", flush=True)
                self.save()
        g = self.margin
        left = int(self.Wc[g : g + self.h, g : g + self.w].sum())
        print(f"done: {len(self.peels)} peels, colored px left: {left:,}", flush=True)
        self.save()

    def save(self):
        placements = [(WHITE, 0, 0)] + self.peels[::-1]
        mcc_a.write_out(self.dd, placements)


if __name__ == "__main__":
    dd = int(sys.argv[1])
    p = Peeler2(dd)
    p.run(max_peels=int(sys.argv[2]) if len(sys.argv) > 2 else None)
    print(f"peels: {len(p.peels)}")
