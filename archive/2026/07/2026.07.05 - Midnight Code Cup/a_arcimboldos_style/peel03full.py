import sys
sys.path.insert(0, ".")
import mcc_a
def write_out(dd, placements):
    with open("out/03p.out", "w") as f:
        f.write(f"{len(placements)}\n")
        for pic, r, c in placements:
            f.write(f"{pic} {r} {c}\n")
mcc_a.write_out = write_out
import solve_peel2
solve_peel2.mcc_a.write_out = write_out
p = solve_peel2.Peeler2(3)
p.run(max_peels=2600)
print(f"peels: {len(p.peels)}")
