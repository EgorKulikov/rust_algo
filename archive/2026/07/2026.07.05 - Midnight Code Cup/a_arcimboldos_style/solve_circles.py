"""Tests 04-06: paint target with small circle cliparts.

Phases:
1. Hex lattice of circles guaranteeing full coverage (circles chosen by SSD
   over their cell).
2. Coordinate descent: for each placement re-choose the circle that minimizes
   error over its *visible* pixels (given z-order), penalizing uncovering.
3. Greedy detail: spend remaining budget placing circles on worst-error spots.
"""
import sys
import time

import numpy as np
from scipy import ndimage

sys.path.insert(0, ".")
import mcc_a


class CircleStack:
    def __init__(self, clips, min_inradius=0.0):
        self.n = len(clips)
        hs = max(c["alpha"].shape[0] for c in clips)
        ws = max(c["alpha"].shape[1] for c in clips)
        self.hs, self.ws = hs, ws
        self.rgb = np.zeros((self.n, hs, ws, 3), dtype=np.float32)
        self.alpha = np.zeros((self.n, hs, ws), dtype=bool)
        self.inrad = np.zeros(self.n)
        self.center = np.zeros((self.n, 2), dtype=int)  # inscribed disk center (r,c)
        self.mean = np.zeros((self.n, 3))
        for i, c in enumerate(clips):
            a = c["alpha"]
            h, w = a.shape
            self.rgb[i, :h, :w] = c["rgb"]
            self.alpha[i, :h, :w] = a
            dt = ndimage.distance_transform_edt(a)
            self.inrad[i] = dt.max()
            self.center[i] = np.unravel_index(np.argmax(dt), dt.shape)
            self.mean[i] = c["rgb"][a].mean(axis=0)


class CirclePainter:
    def __init__(self, dd, cover_r=None, lattice_scale=1.0):
        self.dd = dd
        info = mcc_a.load_input(dd)
        self.n_budget = info["n"]
        self.target = mcc_a.load_target(info["target"]).astype(np.float32)
        self.clips = mcc_a.load_collection(info["collection"])
        self.h, self.w = self.target.shape[:2]
        self.stack = CircleStack(self.clips)
        st = self.stack
        # usable circles for the covering lattice
        if cover_r is None:
            cover_r = np.percentile(st.inrad, 60)  # plenty of choices
        self.cover_ids = np.where(st.inrad >= cover_r)[0]
        self.cover_r = cover_r
        print(f"cover_r={cover_r:.1f}, usable circles: {len(self.cover_ids)}/{st.n}")
        self.placements = []  # (pic, r, c) 1-based pic
        self.canvas = np.full((self.h, self.w, 3), -255, dtype=np.int16)
        self.lattice_scale = lattice_scale

    def render_window(self, r0, r1, c0, c1, skip=None, upto=None):
        """Render placements intersecting window; returns canvas window (int16)
        and topmost-owner index map (-1 none)."""
        win = np.full((r1 - r0, c1 - c0, 3), -255, dtype=np.int16)
        owner = np.full((r1 - r0, c1 - c0), -1, dtype=np.int32)
        for idx, (pic, r, c) in enumerate(self.placements if upto is None else self.placements[:upto]):
            if idx == skip:
                continue
            clip = self.clips[pic - 1]
            ch, cw = clip["alpha"].shape
            if r >= r1 or c >= c1 or r + ch <= r0 or c + cw <= c0:
                continue
            a0, a1 = max(r, r0), min(r + ch, r1)
            b0, b1 = max(c, c0), min(c + cw, c1)
            a = clip["alpha"][a0 - r : a1 - r, b0 - c : b1 - c]
            sub = win[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0]
            sub[a] = clip["rgb"][a0 - r : a1 - r, b0 - c : b1 - c][a]
            osub = owner[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0]
            osub[a] = idx
        return win, owner

    def draw(self, pic, r, c):
        clip = self.clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, self.h)
        c0, c1 = max(c, 0), min(c + cw, self.w)
        if r0 < r1 and c0 < c1:
            a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
            self.canvas[r0:r1, c0:c1][a] = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c][a]

    def best_circle_for(self, tgt_px, cover_mask, ids, topk=40):
        """tgt_px: (H,W,3) target window aligned to stack bbox top-left.
        cover_mask: (H,W) bool pixels that must be matched (visible region).
        Returns (best_id, ssd)."""
        st = self.stack
        hh = min(tgt_px.shape[0], st.hs)
        ww = min(tgt_px.shape[1], st.ws)
        t = tgt_px[:hh, :ww]
        m = cover_mask[:hh, :ww]
        if not m.any():
            return None, 0.0
        tm = t[m].mean(axis=0)
        cand = ids[np.argsort(((st.mean[ids] - tm) ** 2).sum(axis=1))[:topk]]
        a = st.alpha[cand][:, :hh, :ww]
        rgb = st.rgb[cand][:, :hh, :ww]
        diff = ((rgb - t[None]) ** 2).sum(axis=3)
        MISS = 3 * 510.0**2
        ssd = np.where(a, diff, MISS)[:, m].sum(axis=1)
        j = int(np.argmin(ssd))
        return int(cand[j]), float(ssd[j])

    def lattice(self):
        st = self.stack
        R = self.cover_r * self.lattice_scale
        a_sp = R * np.sqrt(3.0)
        row_sp = 1.5 * R
        rows = int(np.ceil(self.h / row_sp)) + 1
        cols = int(np.ceil(self.w / a_sp)) + 1
        print(f"lattice {rows}x{cols} = {rows*cols}")
        count = 0
        for i in range(rows):
            cy = i * row_sp + row_sp / 2 - 2
            for j in range(cols):
                cx = j * a_sp + (a_sp / 2 if i % 2 else 0)
                # window: cell around center
                r0 = int(cy - row_sp); r1 = int(cy + row_sp)
                c0 = int(cx - a_sp / 2); c1 = int(cx + a_sp / 2)
                r0 = max(r0, 0); c0 = max(c0, 0)
                r1 = min(r1, self.h); c1 = min(c1, self.w)
                if r0 >= r1 or c0 >= c1:
                    continue
                # align stack bbox so inscribed center lands on (cy,cx): use avg center
                # choose circle by SSD over cell box
                tgt = self.target[r0:r1, c0:c1]
                tm = tgt.reshape(-1, 3).mean(axis=0)
                ids = self.cover_ids
                cand = ids[np.argsort(((st.mean[ids] - tm) ** 2).sum(axis=1))[:12]]
                best, bssd = None, np.inf
                for ci in cand:
                    ssd = ((st.mean[ci] - tgt) ** 2).sum()
                    if ssd < bssd:
                        best, bssd = ci, ssd
                cr, cc = st.center[best]
                r = int(round(cy)) - cr
                c = int(round(cx)) - cc
                self.placements.append((best + 1, r, c))
                self.draw(best + 1, r, c)
                count += 1
        print(f"lattice placed {count}, err={mcc_a.score(self.target.astype(np.int16), self.canvas):,}")

    def coord_descent(self, passes=2, topk=40):
        st = self.stack
        for pss in range(passes):
            t0 = time.time()
            changed = 0
            for idx in range(len(self.placements)):
                pic, r, c = self.placements[idx]
                clip = self.clips[pic - 1]
                ch, cw = clip["alpha"].shape
                r0 = max(r - 4, 0); c0 = max(c - 4, 0)
                r1 = min(r + st.hs + 4, self.h); c1 = min(c + st.ws + 4, self.w)
                if r0 >= r1 or c0 >= c1:
                    continue
                win, owner = self.render_window(r0, r1, c0, c1)
                vis = owner == idx
                if not vis.any():
                    continue
                # visible mask in stack-aligned coords
                hh, ww = r1 - r0, c1 - c0
                mask = np.zeros((st.hs, st.ws), dtype=bool)
                tgt = np.zeros((st.hs, st.ws, 3), dtype=np.float32)
                rr0 = r - r0; cc0 = c - c0  # clip origin in window coords
                # window region overlapping clip bbox extended to stack size
                a0 = max(0, -rr0); b0 = max(0, -cc0)
                a1 = min(st.hs, hh - rr0); b1 = min(st.ws, ww - cc0)
                if a0 >= a1 or b0 >= b1:
                    continue
                mask[a0:a1, b0:b1] = vis[rr0 + a0 : rr0 + a1, cc0 + b0 : cc0 + b1]
                tgt[a0:a1, b0:b1] = self.target[r0 + rr0 + a0 : r0 + rr0 + a1, c0 + cc0 + b0 : c0 + cc0 + b1]
                new_id, _ = self.best_circle_for(tgt, mask, np.arange(st.n), topk=topk)
                if new_id is not None and new_id + 1 != pic:
                    # verify improvement exactly on window
                    cur_err = ((win.astype(np.float32) - self.target[r0:r1, c0:c1]) ** 2)[vis].sum()
                    clip2 = self.clips[new_id]
                    ch2, cw2 = clip2["alpha"].shape
                    # simulate replacement
                    self.placements[idx] = (new_id + 1, r, c)
                    win2, _ = self.render_window(r0, r1, c0, c1)
                    new_err = ((win2.astype(np.float32) - self.target[r0:r1, c0:c1]) ** 2)[vis].sum()
                    # also check pixels that were visible remain covered overall
                    if new_err < cur_err:
                        changed += 1
                    else:
                        self.placements[idx] = (pic, r, c)
            self.rerender()
            print(f"pass {pss}: changed {changed}, err={mcc_a.score(self.target.astype(np.int16), self.canvas):,} ({time.time()-t0:.0f}s)")

    def rerender(self):
        self.canvas = np.full((self.h, self.w, 3), -255, dtype=np.int16)
        for pic, r, c in self.placements:
            self.draw(pic, r, c)

    def gain_at(self, r0, c0, topk=50):
        """Best (pic, gain) for stack-aligned placement at (r0, c0) on current canvas."""
        st = self.stack
        a1 = min(st.hs, self.h - r0); b1 = min(st.ws, self.w - c0)
        ar0 = max(0, -r0); ac0 = max(0, -c0)
        if a1 <= ar0 or b1 <= ac0:
            return None, -1.0
        tgt = np.zeros((st.hs, st.ws, 3), dtype=np.float32)
        cur = np.zeros((st.hs, st.ws, 3), dtype=np.float32)
        inb = np.zeros((st.hs, st.ws), dtype=bool)
        tgt[ar0:a1, ac0:b1] = self.target[r0 + ar0 : r0 + a1, c0 + ac0 : c0 + b1]
        cur[ar0:a1, ac0:b1] = self.canvas[r0 + ar0 : r0 + a1, c0 + ac0 : c0 + b1]
        inb[ar0:a1, ac0:b1] = True
        cur_err = ((cur - tgt) ** 2).sum(axis=2)
        tm = tgt[inb].mean(axis=0)
        cand = np.argsort(((st.mean - tm) ** 2).sum(axis=1))[:topk]
        diff = ((st.rgb[cand] - tgt[None]) ** 2).sum(axis=3)
        gain = np.where(st.alpha[cand] & inb[None], cur_err[None] - diff, 0.0).sum(axis=(1, 2))
        j = int(np.argmax(gain))
        return int(cand[j]) + 1, float(gain[j])

    def detail(self, max_extra=None, log_every=1000, jitter=6):
        """Greedy: place circles on worst-error blocks until budget or no gain."""
        st = self.stack
        budget = self.n_budget - len(self.placements)
        if max_extra is not None:
            budget = min(budget, max_extra)
        if budget <= 0:
            return
        B = 8
        err = ((self.canvas.astype(np.float32) - self.target) ** 2).sum(axis=2)
        hh = self.h // B * B; ww = self.w // B * B
        eb = err[:hh, :ww].reshape(hh // B, B, ww // B, B).sum(axis=(1, 3))
        t0 = time.time()
        placed = 0
        while placed < budget:
            y, x = np.unravel_index(np.argmax(eb), eb.shape)
            if eb[y, x] <= 0:
                break
            cy, cx = y * B + B // 2, x * B + B // 2
            base_r = cy - st.hs // 2
            base_c = cx - st.ws // 2
            pic, gain = self.gain_at(base_r, base_c)
            best = (gain, pic, base_r, base_c)
            if pic is not None:
                for dr in (-jitter, 0, jitter):
                    for dc in (-jitter, 0, jitter):
                        if dr == 0 and dc == 0:
                            continue
                        p2, g2 = self.gain_at(base_r + dr, base_c + dc)
                        if g2 > best[0]:
                            best = (g2, p2, base_r + dr, base_c + dc)
            gain, pic, r0, c0 = best
            if pic is None or gain <= 0:
                eb[y, x] = 0.0  # dead block
                continue
            self.placements.append((pic, r0, c0))
            self.draw(pic, r0, c0)
            placed += 1
            rr0, cc0 = max(r0, 0), max(c0, 0)
            rr1 = min(r0 + st.hs, self.h); cc1 = min(c0 + st.ws, self.w)
            err[rr0:rr1, cc0:cc1] = ((self.canvas[rr0:rr1, cc0:cc1].astype(np.float32) - self.target[rr0:rr1, cc0:cc1]) ** 2).sum(axis=2)
            by0, by1 = rr0 // B, min((rr1 + B - 1) // B, hh // B)
            bx0, bx1 = cc0 // B, min((cc1 + B - 1) // B, ww // B)
            for by in range(by0, by1):
                for bx in range(bx0, bx1):
                    eb[by, bx] = err[by * B : (by + 1) * B, bx * B : (bx + 1) * B].sum()
            if placed % log_every == 0:
                print(f"detail {placed}/{budget} err={mcc_a.score(self.target.astype(np.int16), self.canvas):,} ({time.time()-t0:.0f}s)", flush=True)
                mcc_a.write_out(self.dd, self.placements)
        print(f"detail placed {placed}, err={mcc_a.score(self.target.astype(np.int16), self.canvas):,}")

    def patch_uncovered(self):
        """Cover remaining holes with circles whose inscribed-disk center is
        placed exactly on the hole pixel (guaranteed cover)."""
        st = self.stack
        while self.n_budget - len(self.placements) > 0:
            holes = np.argwhere(self.canvas[:, :, 0] == -255)
            if len(holes) == 0:
                break
            y, x = holes[0]
            tm = self.target[y, x]
            cand = self.cover_ids[np.argsort(((st.mean[self.cover_ids] - tm) ** 2).sum(axis=1))[:20]]
            best = None
            for ci in cand:
                cr, cc = st.center[ci]
                r0, c0 = y - cr, x - cc
                a1 = min(st.hs, self.h - r0); b1 = min(st.ws, self.w - c0)
                ar0 = max(0, -r0); ac0 = max(0, -c0)
                if a1 <= ar0 or b1 <= ac0:
                    continue
                tgt = self.target[r0 + ar0 : r0 + a1, c0 + ac0 : c0 + b1]
                cur = self.canvas[r0 + ar0 : r0 + a1, c0 + ac0 : c0 + b1].astype(np.float32)
                a = st.alpha[ci][ar0:a1, ac0:b1]
                diff = ((st.rgb[ci][ar0:a1, ac0:b1] - tgt) ** 2).sum(axis=2)
                gain = (np.where(a, ((cur - tgt) ** 2).sum(axis=2) - diff, 0.0)).sum()
                if best is None or gain > best[0]:
                    best = (gain, ci, r0, c0)
            if best is None:
                break
            _, ci, r0, c0 = best
            self.placements.append((int(ci) + 1, r0, c0))
            self.draw(int(ci) + 1, r0, c0)
        print("uncovered after patch:", self.uncovered())

    def uncovered(self):
        return int((self.canvas[:, :, 0] == -255).sum())


if __name__ == "__main__":
    dd = int(sys.argv[1])
    scale = float(sys.argv[2]) if len(sys.argv) > 2 else 1.0
    p = CirclePainter(dd, lattice_scale=scale)
    p.lattice()
    print("uncovered:", p.uncovered())
    p.coord_descent(passes=2)
    p.patch_uncovered()
    p.detail()
    p.coord_descent(passes=1)
    p.detail()
    print("uncovered:", p.uncovered(), "placements:", len(p.placements), "/", p.n_budget)
    mcc_a.write_out(dd, p.placements)
    print("final:", f"{mcc_a.score_file(dd):,}")
