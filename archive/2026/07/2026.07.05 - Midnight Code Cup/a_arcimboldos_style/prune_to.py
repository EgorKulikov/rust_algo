"""Reduce a placement stack to the budget by greedily removing the placement
with the smallest error increase (visible pixels only, exact local eval).

Usage: prune_to.py DD in_path out_path [budget]
"""
import sys

import numpy as np

sys.path.insert(0, ".")
import mcc_a


def owner_map(placements, clips, h, w):
    owner = np.full((h, w), -1, dtype=np.int32)
    for idx, (pic, r, c) in enumerate(placements):
        clip = clips[pic - 1]
        ch, cw = clip["alpha"].shape
        r0, r1 = max(r, 0), min(r + ch, h)
        c0, c1 = max(c, 0), min(c + cw, w)
        if r0 >= r1 or c0 >= c1:
            continue
        a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
        owner[r0:r1, c0:c1][a] = idx
    return owner


def removal_delta(idx, placements, clips, target, owner):
    """err increase if placements[idx] removed (visible px only)."""
    pic, r, c = placements[idx]
    clip = clips[pic - 1]
    h, w = target.shape[:2]
    ch, cw = clip["alpha"].shape
    r0, r1 = max(r, 0), min(r + ch, h)
    c0, c1 = max(c, 0), min(c + cw, w)
    if r0 >= r1 or c0 >= c1:
        return 0.0
    vis = owner[r0:r1, c0:c1] == idx
    if not vis.any():
        return 0.0
    # render what's below idx on this window
    base = np.full((r1 - r0, c1 - c0, 3), -255, dtype=np.int16)
    for j in range(idx):
        p2, rr, cc = placements[j]
        cl = clips[p2 - 1]
        hh, ww = cl["alpha"].shape
        a0, a1 = max(rr, r0), min(rr + hh, r1)
        b0, b1 = max(cc, c0), min(cc + ww, c1)
        if a0 >= a1 or b0 >= b1:
            continue
        a = cl["alpha"][a0 - rr : a1 - rr, b0 - cc : b1 - cc]
        base[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0][a] = cl["rgb"][a0 - rr : a1 - rr, b0 - cc : b1 - cc][a]
    t = target[r0:r1, c0:c1].astype(np.int64)
    cur = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c].astype(np.int64)
    e_with = (((t - cur) ** 2).sum(axis=2) * vis).sum()
    e_without = (((t - base.astype(np.int64)) ** 2).sum(axis=2) * vis).sum()
    return float(e_without - e_with)


def main(dd, in_path, out_path, budget=None):
    info = mcc_a.load_input(dd)
    budget = budget or info["n"]
    target = mcc_a.load_target(info["target"]).astype(np.int16)
    clips = mcc_a.load_collection(info["collection"])
    h, w = target.shape[:2]
    placements = mcc_a.read_out(in_path)
    print(f"{len(placements)} placements -> {budget}")
    while len(placements) > budget:
        owner = owner_map(placements, clips, h, w)
        deltas = [removal_delta(i, placements, clips, target, owner) for i in range(len(placements))]
        # remove a batch of the cheapest non-overlapping removals for speed
        order = np.argsort(deltas)
        removed = set()
        boxes = []
        for i in order:
            if len(placements) - len(removed) <= budget or len(removed) >= 80:
                break
            pic, r, c = placements[i]
            ch, cw = clips[pic - 1]["alpha"].shape
            box = (r, c, r + ch, c + cw)
            if any(not (box[2] <= b[0] or b[2] <= box[0] or box[3] <= b[1] or b[3] <= box[1]) for b in boxes):
                continue
            removed.add(int(i))
            boxes.append(box)
        if not removed:
            i = int(order[0])
            removed = {i}
        placements = [p for j, p in enumerate(placements) if j not in removed]
        print(f"removed {len(removed)}, now {len(placements)}", flush=True)
    err = mcc_a.score(target, mcc_a.render(placements, (h, w), clips))
    print(f"final err={err:,} with {len(placements)}")
    with open(out_path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")


if __name__ == "__main__":
    main(int(sys.argv[1]), sys.argv[2], sys.argv[3], int(sys.argv[4]) if len(sys.argv) > 4 else None)
