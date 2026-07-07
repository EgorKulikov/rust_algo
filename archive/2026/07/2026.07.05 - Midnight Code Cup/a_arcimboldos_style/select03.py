"""Replay full peel (03p), measure per-peel colored-visible px, keep top-999
by that measure (z-order preserved), render + score."""
import sys
sys.path.insert(0, ".")
import numpy as np, mcc_a

info = mcc_a.load_input(3)
target = mcc_a.load_target(info["target"])
clips = mcc_a.load_collection(info["collection"])
h, w = target.shape[:2]
pl = mcc_a.read_out("out/03p.out")
white = pl[0]
peels_bottom_up = pl[1:]          # file order = z-order bottom->top
peels = peels_bottom_up[::-1]     # peel order = top->bottom
colored_t = np.abs(target.astype(np.int16) - 255).max(axis=2) > 0
W = np.ones((h, w), dtype=bool)   # not-yet-peeled
score = []
for (pic, r, c) in peels:
    clip = clips[pic - 1]
    ch, cw = clip["alpha"].shape
    r0, r1 = max(r, 0), min(r + ch, h)
    c0, c1 = max(c, 0), min(c + cw, w)
    if r0 >= r1 or c0 >= c1:
        score.append(0); continue
    a = clip["alpha"][r0 - r : r1 - r, c0 - c : c1 - c]
    act = a & W[r0:r1, c0:c1]
    cnt = int((act & colored_t[r0:r1, c0:c1]).sum())
    score.append(cnt)
    W[r0:r1, c0:c1][a] = False
order = np.argsort(-np.array(score))[:999]
keep = sorted(order.tolist())     # indices in peel (top->bottom) order
kept_bottom_up = [peels[i] for i in keep][::-1]
placements = [white] + kept_bottom_up
canvas = mcc_a.render(placements, (h, w), clips)
err = mcc_a.score(target.astype(np.int16), canvas)
print(f"select-top999-by-colored: err={err:,}")
with open("out/03s.out", "w") as f:
    f.write(f"{len(placements)}\n")
    for pic, r, c in placements:
        f.write(f"{pic} {r} {c}\n")
