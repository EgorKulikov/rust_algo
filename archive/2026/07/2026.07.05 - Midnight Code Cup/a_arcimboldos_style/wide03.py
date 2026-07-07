import os, sys
sys.path.insert(0, ".")
import mcc_a
DD = 3; PATH = "out/03.out"
def write_out(dd, placements):
    with open(PATH, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    return PATH
mcc_a.write_out = write_out
import refine
refine.mcc_a.write_out = write_out
r = refine.Refiner(DD, PATH)
best = r.score()
print(f"start {best:,}", flush=True)
SH = (-32, -16, -8, -4, -2, -1, 0, 1, 2, 4, 8, 16, 32)
for p in range(8):
    imp, ch, dt = r.refine_pass(shifts=SH, topk=27, pad=40, size_tol=30.0)
    s = r.score()
    print(f"pass {p}: changed {ch}, err={s:,} ({dt:.0f}s)", flush=True)
    if s < best:
        write_out(DD, r.placements); best = s
    if ch == 0:
        break
print("final:", f"{best:,}")
