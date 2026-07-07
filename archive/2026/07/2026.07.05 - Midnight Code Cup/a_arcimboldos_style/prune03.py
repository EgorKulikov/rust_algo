"""Prune placements whose removal leaves the rendered canvas error unchanged
(or better). Iterates until fixpoint. Usage: prune03.py DD [path]"""
import sys

import numpy as np

sys.path.insert(0, ".")
import mcc_a


def main(dd, path=None):
    info = mcc_a.load_input(dd)
    target = mcc_a.load_target(info["target"]).astype(np.int16)
    clips = mcc_a.load_collection(info["collection"])
    h, w = target.shape[:2]
    path = path or f"out/{dd:02d}.out"
    placements = mcc_a.read_out(path)
    print(f"{len(placements)} placements, err={mcc_a.score(target, mcc_a.render(placements, (h, w), clips)):,}")

    changed = True
    rounds = 0
    while changed:
        changed = False
        rounds += 1
        # owner map: topmost placement per pixel
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
        keep = []
        for idx in range(len(placements) - 1, -1, -1):
            pic, r, c = placements[idx]
            vis = owner == idx
            if not vis.any():
                # fully hidden -> free removal
                placements.pop(idx)
                changed = True
                continue
            # removal exposes below; check err delta on visible px
            sub = placements[:idx] + placements[idx + 1 :]
            ys, xs = np.where(vis)
            r0, r1 = ys.min(), ys.max() + 1
            c0, c1 = xs.min(), xs.max() + 1
            win_with = mcc_a.render(placements, (h, w), clips)[r0:r1, c0:c1] if False else None
            # local render without idx
            base = np.full((r1 - r0, c1 - c0, 3), -255, dtype=np.int16)
            for j, (p2, rr, cc) in enumerate(placements):
                if j == idx:
                    continue
                clip = clips[p2 - 1]
                ch, cw = clip["alpha"].shape
                a0, a1 = max(rr, r0), min(rr + ch, r1)
                b0, b1 = max(cc, c0), min(cc + cw, c1)
                if a0 >= a1 or b0 >= b1:
                    continue
                a = clip["alpha"][a0 - rr : a1 - rr, b0 - cc : b1 - cc]
                base[a0 - r0 : a1 - r0, b0 - c0 : b1 - c0][a] = clip["rgb"][a0 - rr : a1 - rr, b0 - cc : b1 - cc][a]
            vw = vis[r0:r1, c0:c1]
            t = target[r0:r1, c0:c1].astype(np.int64)
            clip = clips[pic - 1]
            cur = clip["rgb"][r0 - r : r1 - r, c0 - c : c1 - c].astype(np.int64)
            e_with = (((t - cur) ** 2).sum(axis=2) * vw).sum()
            e_without = (((t - base.astype(np.int64)) ** 2).sum(axis=2) * vw).sum()
            if e_without <= e_with:
                placements.pop(idx)
                changed = True
        print(f"round {rounds}: {len(placements)} left")
        if rounds > 6:
            break
    err = mcc_a.score(target, mcc_a.render(placements, (h, w), clips))
    print(f"final {len(placements)} placements, err={err:,}")
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")


if __name__ == "__main__":
    main(int(sys.argv[1]), sys.argv[2] if len(sys.argv) > 2 else None)
