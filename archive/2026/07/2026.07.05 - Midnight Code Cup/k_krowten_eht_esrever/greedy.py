"""Greedy per-cell optimizer, seeded from the current best image.

For each cell (coarse -> fine) we try flipping it to the opposite color and keep
the change iff the score strictly improves. Works whenever the beast gives a
smooth-ish signal (Beast B). Runs forever, refining, until a key drops.
"""
import sys
import numpy as np
import mcc

N = mcc.N


def load_seed(beast):
    if beast.best_arr is not None:
        return beast.best_arr.copy()
    return np.zeros((N, N), bool)


def greedy(beast, cell_schedule=(32, 16, 8, 4, 2), passes=6):
    cur = load_seed(beast)
    # establish current score baseline
    cur_score, _ = beast.query(cur, tag="greedy:seed")
    rng = np.random.default_rng(1)
    for cell in cell_schedule:
        g = N // cell
        for p in range(passes):
            cells = [(i, j) for i in range(g) for j in range(g)]
            rng.shuffle(cells)
            improved = 0
            for (i, j) in cells:
                if beast.key:
                    return cur
                y0, x0 = i * cell, j * cell
                trial = cur.copy()
                trial[y0:y0 + cell, x0:x0 + cell] ^= True
                s, k = beast.query(trial, tag=f"greedy:c{cell}p{p}:{i},{j}")
                if s > cur_score + 1e-9:
                    cur = trial
                    cur_score = s
                    improved += 1
            print(f"[{beast.name}] cell{cell} pass{p}: score={cur_score:.4f} "
                  f"improved={improved}/{g*g}")
            if improved == 0:
                break
    return cur


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "b"
    beast = mcc.Beast(name)
    print(f"[{name}] start greedy, resume best={beast.best_score:.4f} key={beast.key}")
    greedy(beast)
    print(f"[{name}] done best={beast.best_score:.4f} key={beast.key}")
