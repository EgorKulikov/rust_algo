"""Z-order-aware coordinate descent on an existing .out solution.

For each placement idx: within its local window compute
  B      = render of placements below idx (-255 where none),
  Later  = mask of pixels covered by placements above idx (immutable),
then for candidate clipart ci at offset (r,c):
  err = sum_{opaque & ~Later} (T-L)^2 + sum_{~opaque & ~Later} (T-B)^2.
Rank candidates (current clipart at shifts + top-K same-size-class cliparts
by visible mean color) on 4x-downscaled arrays, verify winners exactly.
Also tries dropping the placement entirely.
"""
import sys
import time

import numpy as np

sys.path.insert(0, ".")
import mcc_a

MISS2 = 3 * 510.0**2


class Refiner:
    def __init__(self, dd, out_path=None):
        self.dd = dd
        info = mcc_a.load_input(dd)
        self.n = info["n"]
        self.target = mcc_a.load_target(info["target"]).astype(np.float32)
        self.clips = mcc_a.load_collection(info["collection"])
        self.h, self.w = self.target.shape[:2]
        self.placements = mcc_a.read_out(out_path or f"out/{dd:02d}.out")
        self.means = np.array([
            c["rgb"][c["alpha"]].mean(axis=0) if c["alpha"].any() else [0, 0, 0]
            for c in self.clips
        ])
        self.areas = np.array([c["alpha"].sum() for c in self.clips], dtype=np.float64)

    def build_index(self, cell=128):
        """Grid index: cell -> sorted list of placement indices intersecting it."""
        self.cell = cell
        self.index = {}
        for j, (pic, r, c) in enumerate(self.placements):
            ch, cw = self.clips[pic - 1]["alpha"].shape
            for gy in range(max(r, 0) // cell, min(r + ch, self.h) // cell + 1):
                for gx in range(max(c, 0) // cell, min(c + cw, self.w) // cell + 1):
                    self.index.setdefault((gy, gx), []).append(j)

    def near(self, r0, r1, c0, c1):
        seen = set()
        for gy in range(r0 // self.cell, r1 // self.cell + 1):
            for gx in range(c0 // self.cell, c1 // self.cell + 1):
                seen.update(self.index.get((gy, gx), ()))
        return sorted(seen)

    def below_later(self, idx, r0, r1, c0, c1):
        B = np.full((r1 - r0, c1 - c0, 3), -255, dtype=np.float32)
        later = np.zeros((r1 - r0, c1 - c0), dtype=bool)
        for j in self.near(r0, r1, c0, c1):
            pic, r, c = self.placements[j]
            if j == idx:
                continue
            clip = self.clips[pic - 1]
            ch, cw = clip["alpha"].shape
            if r >= r1 or c >= c1 or r + ch <= r0 or c + cw <= c0:
                continue
            a0, a1 = max(r, r0), min(r + ch, r1)
            b0, b1 = max(c, c0), min(c + cw, c1)
            a = clip["alpha"][a0 - r : a1 - r, b0 - c : b1 - c]
            if j < idx:
                sub = B[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0]
                sub[a] = clip["rgb"][a0 - r : a1 - r, b0 - c : b1 - c][a].astype(np.float32)
            else:
                later[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0] |= a
        return B, later

    def cand_err(self, pic, r, c, r0, r1, c0, c1, T, B, later):
        """Exact err in window if placement = (pic,r,c). pic=None -> dropped."""
        base = np.where(later[:, :, None], 0.0, (T - B) ** 2)
        err = base.sum()
        if pic is None:
            return err
        clip = self.clips[pic - 1]
        ch, cw = clip["alpha"].shape
        a0, a1 = max(r, r0), min(r + ch, r1)
        b0, b1 = max(c, c0), min(c + cw, c1)
        if a0 >= a1 or b0 >= b1:
            return err
        a = clip["alpha"][a0 - r : a1 - r, b0 - c : b1 - c] & ~later[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0]
        t = T[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0]
        l = clip["rgb"][a0 - r : a1 - r, b0 - c : b1 - c].astype(np.float32)
        d_new = ((t - l) ** 2).sum(axis=2)
        d_old = ((t - B[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0]) ** 2).sum(axis=2)
        return err + (np.where(a, d_new - d_old, 0.0)).sum()

    def refine_pass(self, shifts=(-6, -3, -1, 0, 1, 3, 6), topk=12, pad=8, allow_drop=True, size_tol=3.0):
        improved = 0.0
        n_changed = 0
        t0 = time.time()
        self.build_index()
        for idx in range(len(self.placements) - 1, -1, -1):
            pic, r, c = self.placements[idx]
            clip = self.clips[pic - 1]
            ch, cw = clip["alpha"].shape
            r0 = max(r - pad, 0); c0 = max(c - pad, 0)
            r1 = min(r + ch + pad, self.h); c1 = min(c + cw + pad, self.w)
            if r0 >= r1 or c0 >= c1:
                continue
            T = self.target[r0:r1, c0:c1]
            B, later = self.below_later(idx, r0, r1, c0, c1)
            cur = self.cand_err(pic, r, c, r0, r1, c0, c1, T, B, later)
            best = (cur, pic, r, c)
            # same clipart, shifted
            for dr in shifts:
                for dc in shifts:
                    if dr == 0 and dc == 0:
                        continue
                    e = self.cand_err(pic, r + dr, c + dc, r0, r1, c0, c1, T, B, later)
                    if e < best[0]:
                        best = (e, pic, r + dr, c + dc)
            # alternative cliparts of similar size, same center
            vis = ~later
            tm = T[vis].mean(axis=0) if vis.any() else np.zeros(3)
            ratio = self.areas / max(self.areas[pic - 1], 1.0)
            ok = (ratio > 1 / size_tol) & (ratio < size_tol)
            ids = np.where(ok)[0]
            if len(ids):
                cand = ids[np.argsort(((self.means[ids] - tm) ** 2).sum(axis=1))[:topk]]
                cy, cx = r + ch / 2, c + cw / 2
                for ci in cand:
                    if ci + 1 == pic:
                        continue
                    ch2, cw2 = self.clips[ci]["alpha"].shape
                    rr = int(round(cy - ch2 / 2)); cc = int(round(cx - cw2 / 2))
                    # candidate must fit inside the evaluated window, else its
                    # effect outside the window is invisible to cand_err
                    if not (max(rr, 0) >= r0 and max(cc, 0) >= c0
                            and min(rr + ch2, self.h) <= r1 and min(cc + cw2, self.w) <= c1):
                        continue
                    for dr, dc in ((0, 0), (-3, 0), (3, 0), (0, -3), (0, 3)):
                        e = self.cand_err(ci + 1, rr + dr, cc + dc, r0, r1, c0, c1, T, B, later)
                        if e < best[0]:
                            best = (e, ci + 1, rr + dr, cc + dc)
            if allow_drop:
                e = self.cand_err(None, 0, 0, r0, r1, c0, c1, T, B, later)
                if e < best[0]:
                    best = (e, None, 0, 0)
            if best[1] != pic or (best[1] is not None and (best[2], best[3]) != (r, c)):
                improved += cur - best[0]
                n_changed += 1
                if best[1] is None:
                    self.placements.pop(idx)
                    self.build_index()
                    continue
                self.placements[idx] = (best[1], best[2], best[3])
        return improved, n_changed, time.time() - t0

    def score(self):
        canvas = mcc_a.render(self.placements, (self.h, self.w), self.clips)
        return mcc_a.score(self.target.astype(np.int16), canvas)

    def run(self, passes=4, **kw):
        s0 = self.score()
        print(f"start err={s0:,}")
        for p in range(passes):
            imp, ch, dt = self.refine_pass(**kw)
            s = self.score()
            print(f"pass {p}: changed {ch}, err={s:,} ({dt:.0f}s)", flush=True)
            mcc_a.write_out(self.dd, self.placements)
            if ch == 0:
                break
        return self.score()


if __name__ == "__main__":
    dd = int(sys.argv[1])
    passes = int(sys.argv[2]) if len(sys.argv) > 2 else 4
    r = Refiner(dd)
    final = r.run(passes=passes)
    print("final:", f"{final:,}")
