import os, sys
sys.path.insert(0, ".")
import mcc_a
def write_out(dd, placements):
    path = os.path.join(mcc_a.OUT, "09g.out")
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    return path
mcc_a.write_out = write_out
import solve_collage
solve_collage.mcc_a.write_out = write_out
g = solve_collage.CollageGreedy(9, d=6, budget=1500, resume=os.path.join(mcc_a.OUT, "09g.out"))
g.run(refine_step=3)
g.save()
