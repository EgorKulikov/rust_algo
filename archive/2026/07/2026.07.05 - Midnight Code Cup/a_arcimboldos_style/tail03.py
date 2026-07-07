import sys
sys.path.insert(0, ".")
import mcc_a
from solve_letters import LetterGreedy
g = LetterGreedy(3, d=2, resume_path="out/03.out")
g.run(log_every=10)
mcc_a.write_out(3, g.placements)
print("final:", f"{mcc_a.score_file(3):,}", "placements:", len(g.placements))
