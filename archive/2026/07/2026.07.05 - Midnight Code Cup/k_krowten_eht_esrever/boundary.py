"""Boundary-focused refinement for deterministic Beast A.

After 4px greedy plateaus, remaining score lives in sub-4px detail at the edges
of A's pieces. Instead of a full fine pass over black voids, we only test cells
in the band around the current white/black boundary: flip -> keep iff the (noise-
free) score improves. Coarse-to-fine cell sizes, several passes, resumes from best.
"""
import sys
import numpy as np
from PIL import Image, ImageFilter
import mcc

N = mcc.N


def band(white, m):
    im = Image.fromarray((white * 255).astype("uint8"))
    dil = np.asarray(im.filter(ImageFilter.MaxFilter(2 * m + 1))) > 127
    ero = np.asarray(im.filter(ImageFilter.MinFilter(2 * m + 1))) > 127
    return dil & ~ero


def refine(beast, cell_schedule=(2, 1), margin=6, passes=8):
    cur = beast.best_arr.copy()
    cur_score, _ = beast.query(cur, tag="bnd:seed")
    rng = np.random.default_rng(3)
    for c in cell_schedule:
        for p in range(passes):
            b = band(cur, margin)
            # collect c-aligned cells overlapping the band
            cells = set()
            ys, xs = np.where(b)
            for y, x in zip(ys, xs):
                cells.add((y // c * c, x // c * c))
            cells = list(cells)
            rng.shuffle(cells)
            improved = 0
            for (y, x) in cells:
                if beast.key:
                    return cur
                trial = cur.copy()
                trial[y:y + c, x:x + c] ^= True
                s, _ = beast.query(trial, tag=f"bnd:c{c}p{p}")
                if s > cur_score + 1e-9:
                    cur = trial
                    cur_score = s
                    improved += 1
            print(f"[{beast.name}] band c{c} p{p}: cells={len(cells)} "
                  f"improved={improved} score={cur_score:.4f}", flush=True)
            if improved == 0:
                break
    return cur


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "a"
    beast = mcc.Beast(name)
    print(f"[{name}] boundary refine seed={beast.best_score:.4f}", flush=True)
    while not beast.key:
        before = beast.best_score
        refine(beast)
        if beast.best_score <= before + 1e-6:
            print(f"[{name}] no further boundary gain at {beast.best_score:.4f}", flush=True)
            break
    print(f"[{name}] done best={beast.best_score:.4f} key={beast.key}", flush=True)
