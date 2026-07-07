"""Fresh greedy base layer for test 09 (big flowers) to out/09g.out."""
import os
import sys

sys.path.insert(0, ".")
import mcc_a


def write_out(dd, placements):
    os.makedirs(mcc_a.OUT, exist_ok=True)
    path = os.path.join(mcc_a.OUT, "09g.out")
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    return path


mcc_a.write_out = write_out

import solve_collage

solve_collage.mcc_a.write_out = write_out
g = solve_collage.CollageGreedy(9, d=6, budget=1800)
g.run(refine_step=3)
g.save()
