"""Membership optimization for t03: full peel stack (03p) defines candidate
letters with fixed z-order. Choose subset of <=999 minimizing rendered error.

Iterative exchange: compute for each candidate letter its marginal value
(err delta if toggled) against the current selection render, then flip the
best moves (add valuable dropped, remove worthless kept), re-render, repeat.
"""
import sys

import numpy as np

sys.path.insert(0, ".")
import mcc_a

info = mcc_a.load_input(3)
target = mcc_a.load_target(info["target"]).astype(np.int16)
clips = mcc_a.load_collection(info["collection"])
h, w = target.shape[:2]
pl = mcc_a.read_out("out/03p.out")
white = pl[0]
letters = pl[1:]  # bottom-up z-order
N = len(letters)
BUDGET = 999

# current selection from 03s.out (skip white)
cur_set = set()
sel = mcc_a.read_out("out/03.out")[1:]
pos_index = {}
for i, p in enumerate(letters):
    pos_index.setdefault(p, []).append(i)
used = {}
for p in sel:
    idxs = pos_index.get(p, [])
    k = used.get(p, 0)
    if k < len(idxs):
        cur_set.add(idxs[k])
        used[p] = k + 1
import numpy as _np, shutil as _sh
_rng = _np.random.default_rng()
print(f"basin hopping from: {len(cur_set)}")


def render_sel(sel_set):
    placements = [white] + [letters[i] for i in sorted(sel_set)]
    return mcc_a.render(placements, (h, w), clips)


def owner_of(sel_sorted):
    owner = np.full((h, w), -1, dtype=np.int32)
    for idx in sel_sorted:
        pic, r, c = letters[idx]
        clip = clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, h)
        c0, c1 = max(c, 0), min(c + cw, w)
        if r0 >= r1 or c0 >= c1:
            continue
        a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
        owner[r0:r1, c0:c1][a] = idx
    return owner



best_set = set(cur_set)
best_e = None
for hop in range(1000):
  if hop > 0:
    cur_set = set(best_set)
    _kept = sorted(cur_set)
    for _i in _rng.choice(_kept, size=20, replace=False): cur_set.discard(int(_i))
    _rest = [i for i in range(N) if i not in cur_set]
    for _i in _rng.choice(_rest, size=20, replace=False): cur_set.add(int(_i))
  for it in range(200):
    canvas = render_sel(cur_set)
    err = mcc_a.score(target, canvas)
    print(f"iter {it}: err={err:,} sel={len(cur_set)}", flush=True)
    sel_sorted = sorted(cur_set)
    owner = owner_of(sel_sorted)
    errmap = ((target.astype(np.int64) - canvas) ** 2).sum(axis=2)
    moves = []  # (delta, idx, kind)
    for idx in range(N):
        pic, r, c = letters[idx]
        clip = clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, h)
        c0, c1 = max(c, 0), min(c + cw, w)
        if r0 >= r1 or c0 >= c1:
            continue
        a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
        if idx in cur_set:
            # drop: visible px get exposed to below-render; approximate the
            # exposed content via rerender of window without idx
            vis = (owner[r0:r1, c0:c1] == idx)
            if not vis.any():
                moves.append((0.0, idx, "drop"))
                continue
            sub = [j for j in sel_sorted if j != idx]
            base = np.full((r1 - r0, c1 - c0, 3), -255, dtype=np.int16)
            for j in sub:
                p2, rr, cc = letters[j]
                cl = clips[p2 - 1]
                hh, ww = cl["alpha"].shape
                a0, a1 = max(rr, r0), min(rr + hh, r1)
                b0, b1 = max(cc, c0), min(cc + ww, c1)
                if a0 >= a1 or b0 >= b1:
                    continue
                aa = cl["alpha"][a0 - rr : a1 - rr, b0 - cc : b1 - cc]
                base[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0][aa] = cl["rgb"][a0 - rr : a1 - rr, b0 - cc : b1 - cc][aa]
            t = target[r0:r1, c0:c1].astype(np.int64)
            e_now = (errmap[r0:r1, c0:c1] * vis).sum()
            e_after = (((t - base.astype(np.int64)) ** 2).sum(axis=2) * vis).sum()
            moves.append((float(e_after - e_now), idx, "drop"))
        else:
            # add at z-slot idx: covers px where no kept letter above idx
            above = (owner[r0:r1, c0:c1] > idx)
            act = a & ~above
            if not act.any():
                continue
            t = target[r0:r1, c0:c1].astype(np.int64)
            l = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c].astype(np.int64)
            e_now = (errmap[r0:r1, c0:c1] * act).sum()
            e_new = (((t - l) ** 2).sum(axis=2) * act).sum()
            moves.append((float(e_new - e_now), idx, "add"))
    # apply: best adds while budget, plus drops that free err, pair worst-add
    adds = sorted([m for m in moves if m[2] == "add" and m[0] < 0])
    drops = sorted([m for m in moves if m[2] == "drop"])
    changed = 0
    boxes = []
    def box_of(idx):
        pic, r, c = letters[idx]
        ch, cw = clips[pic - 1]["alpha"].shape
        return (r, c, r + ch, c + cw)
    def overlaps(b):
        return any(not (b[2] <= x[0] or x[2] <= b[0] or b[3] <= x[1] or x[3] <= b[1]) for x in boxes)
    di = 0
    for delta, idx, _ in adds:
        b = box_of(idx)
        if overlaps(b):
            continue
        # need budget: drop cheapest non-overlapping if full
        while len(cur_set) >= BUDGET and di < len(drops):
            d_delta, d_idx, _ = drops[di]
            di += 1
            if d_idx not in cur_set:
                continue
            db = box_of(d_idx)
            if overlaps(db):
                continue
            if d_delta + delta < 0:
                cur_set.discard(d_idx)
                boxes.append(db)
                break
            else:
                di = len(drops)
        if len(cur_set) >= BUDGET:
            break
        cur_set.add(idx)
        boxes.append(b)
        changed += 1
        if changed >= 60:
            break
    if changed == 0:
        break
  canvas = render_sel(cur_set)
  err = mcc_a.score(target, canvas)
  if best_e is None or err < best_e:
    best_e = err
    best_set = set(cur_set)
    placements = [white] + [letters[i] for i in sorted(best_set)]
    with open("out/03s.out", "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    print(f"hop {hop}: NEW BEST {err:,}", flush=True)
  else:
    print(f"hop {hop}: {err:,} (best {best_e:,})", flush=True)

canvas = render_sel(best_set)
err = mcc_a.score(target, canvas)
print(f"final err={err:,} sel={len(cur_set)}")
placements = [white] + [letters[i] for i in sorted(cur_set)]
with open("out/03s.out", "w") as f:
    f.write(f"{len(placements)}\n")
    for pic, r, c in placements:
        f.write(f"{pic} {r} {c}\n")
