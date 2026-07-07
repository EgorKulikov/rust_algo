"""Fresh full-budget savings-map greedy for test 04, checkpointing to out/04g.out
so it never fights the incumbent chain over out/04.out."""
import os
import sys

sys.path.insert(0, ".")
import mcc_a

_orig = mcc_a.write_out


def write_out(dd, placements):
    os.makedirs(mcc_a.OUT, exist_ok=True)
    path = os.path.join(mcc_a.OUT, "04g.out")
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    return path


mcc_a.write_out = write_out

import solve_collage

solve_collage.mcc_a.write_out = write_out
g = solve_collage.CollageGreedy(4, d=4, budget=10000)
g.run(refine_step=2)
g.save()
