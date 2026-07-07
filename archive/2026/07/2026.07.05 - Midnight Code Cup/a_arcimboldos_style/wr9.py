import os, sys
sys.path.insert(0, ".")
import mcc_a
PATH = "out/09w.out"
def write_out(dd, placements):
    with open(PATH, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    return PATH
mcc_a.write_out = write_out
import refine
refine.mcc_a.write_out = write_out
r = refine.Refiner(9, PATH)
best = r.score()
print(f"start {best:,}", flush=True)
for p in range(8):
    imp, ch, dt = r.refine_pass(shifts=(-40, -20, -8, 0, 8, 20, 40), topk=12, pad=48)
    s = r.score()
    print(f"pass {p}: changed {ch}, err={s:,} ({dt:.0f}s)", flush=True)
    if s < best:
        write_out(9, r.placements); best = s
    if ch == 0:
        break
print("final:", f"{best:,}")
