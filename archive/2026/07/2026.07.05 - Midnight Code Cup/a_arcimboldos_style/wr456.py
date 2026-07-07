import os, sys
sys.path.insert(0, ".")
import mcc_a
DD = int(sys.argv[1]); PATH = sys.argv[2]
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
for p in range(10):
    imp, ch, dt = r.refine_pass(shifts=(-16, -10, -6, -3, -1, 0, 1, 3, 6, 10, 16), topk=25, pad=24)
    s = r.score()
    print(f"pass {p}: changed {ch}, err={s:,} ({dt:.0f}s)", flush=True)
    if s < best:
        write_out(DD, r.placements); best = s
    if ch == 0:
        break
print("final:", f"{best:,}")
