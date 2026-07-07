"""Simulated annealing on t03 letter-subset membership (z-order fixed by the
full peel stack). Move = toggle one letter (paired with a counter-toggle when
at budget). Delta evaluated exactly on the letter's window; owner map and
canvas updated incrementally on accept.

Usage: sa03.py [minutes] [seed]
"""
import sys
import time

import numpy as np

sys.path.insert(0, ".")
import mcc_a

rng = np.random.default_rng(int(sys.argv[2]) if len(sys.argv) > 2 else 12345)
MINUTES = float(sys.argv[1]) if len(sys.argv) > 1 else 45.0

info = mcc_a.load_input(3)
target = mcc_a.load_target(info["target"]).astype(np.int16)
clips = mcc_a.load_collection(info["collection"])
h, w = target.shape[:2]
pl = mcc_a.read_out("out/03p.out")
white = pl[0]
letters = pl[1:]
N = len(letters)
BUDGET = 999

sel = mcc_a.read_out("out/03.out")[1:]
pos_index = {}
for i, p in enumerate(letters):
    pos_index.setdefault(p, []).append(i)
used = {}
cur = np.zeros(N, dtype=bool)
for p in sel:
    idxs = pos_index.get(p, [])
    k = used.get(p, 0)
    if k < len(idxs):
        cur[idxs[k]] = True
        used[p] = k + 1

boxes = []
for (pic, r, c) in letters:
    ch, cw = clips[pic - 1]["alpha"].shape
    boxes.append((max(r, 0), max(c, 0), min(r + ch, h), min(c + cw, w)))


def render_from(sel_mask):
    placements = [white] + [letters[i] for i in range(N) if sel_mask[i]]
    return mcc_a.render(placements, (h, w), clips)


# grid index: letters intersecting each 128px cell
CELL = 128
grid = {}
for j, b in enumerate(boxes):
    for gy in range(b[0] // CELL, (b[2] - 1) // CELL + 1):
        for gx in range(b[1] // CELL, (b[3] - 1) // CELL + 1):
            grid.setdefault((gy, gx), []).append(j)


def near(r0, c0, r1, c1):
    seen = set()
    for gy in range(r0 // CELL, (r1 - 1) // CELL + 1):
        for gx in range(c0 // CELL, (c1 - 1) // CELL + 1):
            seen.update(grid.get((gy, gx), ()))
    return sorted(seen)


def local_render(sel_mask, r0, c0, r1, c1, skip=-1, force=-1):
    win = np.full((r1 - r0, c1 - c0, 3), -255, dtype=np.int16)
    for j in near(r0, c0, r1, c1):
        if not (sel_mask[j] or j == force) or j == skip:
            continue
        b = boxes[j]
        if b[0] >= r1 or b[2] <= r0 or b[1] >= c1 or b[3] <= c0:
            continue
        pic, r, c = letters[j]
        cl = clips[pic - 1]
        a0, a1 = max(b[0], r0), min(b[2], r1)
        b0_, b1_ = max(b[1], c0), min(b[3], c1)
        a = cl["alpha"][a0 - r : a1 - r, b0_ - c : b1_ - c]
        win[a0 - r0 : a1 - r0, b0_ - c0 : b1_ - c0][a] = cl["rgb"][a0 - r : a1 - r, b0_ - c : b1_ - c][a]
    # white sheet is below everything: replace uncovered with white
    unc = win[:, :, 0] == -255
    win[unc] = 255
    return win


canvas = render_from(cur)
err = mcc_a.score(target, canvas)
best_err = err
print(f"start err={err:,}", flush=True)

n_sel = int(cur.sum())
t0 = time.time()
accepted = 0
tries = 0
T0 = 200000.0
while time.time() - t0 < MINUTES * 60:
    tries += 1
    frac = (time.time() - t0) / (MINUTES * 60)
    T = T0 * (1 - frac) ** 2 + 1.0
    i = int(rng.integers(N))
    b = boxes[i]
    if b[0] >= b[2] or b[1] >= b[3]:
        continue
    r0, c0, r1, c1 = b
    t = target[r0:r1, c0:c1].astype(np.int64)
    cur_win = local_render(cur, r0, c0, r1, c1)
    e_now = ((t - cur_win.astype(np.int64)) ** 2).sum()
    if cur[i]:
        new_win = local_render(cur, r0, c0, r1, c1, skip=i)
    else:
        new_win = local_render(cur, r0, c0, r1, c1, force=i)
    e_new = ((t - new_win.astype(np.int64)) ** 2).sum()
    delta = float(e_new - e_now)
    if not cur[i] and n_sel >= BUDGET:
        # must also drop something: pick the cheapest currently-selected in a
        # random sample of 40
        cand = rng.choice(np.where(cur)[0], size=min(40, n_sel), replace=False)
        best_d = None
        for j in cand:
            bj = boxes[j]
            if bj[0] >= bj[2] or bj[1] >= bj[3]:
                d = 0.0
            else:
                tj = target[bj[0] : bj[2], bj[1] : bj[3]].astype(np.int64)
                wj = local_render(cur, *bj)
                wj2 = local_render(cur, *bj, skip=j)
                d = float(((tj - wj2.astype(np.int64)) ** 2).sum() - ((tj - wj.astype(np.int64)) ** 2).sum())
            if best_d is None or d < best_d[0]:
                best_d = (d, int(j))
        delta += best_d[0]
        drop_j = best_d[1]
    else:
        drop_j = None
    if delta < 0 or rng.random() < np.exp(-delta / T):
        if cur[i]:
            cur[i] = False
            n_sel -= 1
        else:
            cur[i] = True
            n_sel += 1
            if drop_j is not None:
                cur[drop_j] = False
                n_sel -= 1
        err += delta
        accepted += 1
        if err < best_err - 1000:
            best_err = err
            placements = [white] + [letters[j] for j in range(N) if cur[j]]
            with open("out/03s.out", "w") as f:
                f.write(f"{len(placements)}\n")
                for pic, r, c in placements:
                    f.write(f"{pic} {r} {c}\n")
    if tries % 5000 == 0:
        real = mcc_a.score(target, render_from(cur))
        err = real  # resync drift
        print(f"{tries} tries, {accepted} acc, err={real:,} T={T:.0f} ({time.time()-t0:.0f}s)", flush=True)

real = mcc_a.score(target, render_from(cur))
print(f"end err={real:,} best={best_err:,}")
