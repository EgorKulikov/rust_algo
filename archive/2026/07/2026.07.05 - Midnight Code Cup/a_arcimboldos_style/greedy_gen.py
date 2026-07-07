"""Generic fresh savings-map greedy to a side file out/DDg.out.
Usage: greedy_gen.py DD d budget [refine_step]
"""
import os
import sys

sys.path.insert(0, ".")
import mcc_a

DD = int(sys.argv[1])


def write_out(dd, placements):
    os.makedirs(mcc_a.OUT, exist_ok=True)
    path = os.path.join(mcc_a.OUT, f"{DD:02d}g.out")
    with open(path, "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
    return path


mcc_a.write_out = write_out

import solve_collage

solve_collage.mcc_a.write_out = write_out
g = solve_collage.CollageGreedy(DD, d=int(sys.argv[2]), budget=int(sys.argv[3]))
g.run(refine_step=int(sys.argv[4]) if len(sys.argv) > 4 else 2)
g.save()
