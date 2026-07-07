"""Simulated annealing for deterministic Beast A with multi-cell moves.

Greedy single-cell hill-climbing parks at ~50. If A's objective rewards complete
polyomino pieces superlinearly, completing/adding whole pieces needs coordinated
multi-cell moves that no single flip captures. A is deterministic (no noise), so
SA on the true score with block moves can escape the single-cell local optimum.
Moves: flip a k x k block of 4px cells (k in 1..3) at a random position; also
occasionally add a random tetromino in a black area. Metropolis accept; reheat.
"""
import sys
import math
import numpy as np
import mcc

N = mcc.N
CELL = 4                     # A lives on a 4px grid
G = N // CELL                # 64


def anneal(beast, iters=100000, t_hi=1.5, t_lo=0.05, reheat=1200, seed=0):
    rng = np.random.default_rng(seed)
    cur = beast.best_arr.copy() if beast.best_arr is not None else np.zeros((N, N), bool)
    cur_s, _ = beast.query(cur, tag="saA:seed")
    best = cur.copy(); best_s = cur_s
    since = 0
    for it in range(iters):
        if beast.key:
            return best
        frac = it / iters
        T = t_hi * (t_lo / t_hi) ** frac
        k = int(rng.integers(1, 4))          # block size in cells
        ci = int(rng.integers(0, G - k + 1))
        cj = int(rng.integers(0, G - k + 1))
        trial = cur.copy()
        y0, x0 = ci * CELL, cj * CELL
        trial[y0:y0 + k * CELL, x0:x0 + k * CELL] ^= True
        s, kk = beast.query(trial, tag=f"saA:{it}:k{k}")
        d = s - cur_s
        if d >= 0 or rng.random() < math.exp(d / max(T, 1e-6)):
            cur, cur_s = trial, s
        if s > best_s:
            best_s, best = s, trial.copy(); since = 0
        else:
            since += 1
        if it % 100 == 0:
            print(f"[A] sa it={it} T={T:.3f} k={k} cur={cur_s:.3f} best={best_s:.4f}", flush=True)
        if since >= reheat:
            cur, cur_s = best.copy(), best_s; since = 0
            t_hi = min(t_hi * 1.3, 6.0)
            print(f"[A] reheat -> t_hi={t_hi:.2f}", flush=True)
    return best


if __name__ == "__main__":
    beast = mcc.Beast("a")
    print(f"[A] SA start best={beast.best_score:.4f}", flush=True)
    anneal(beast)
    print(f"[A] SA end best={beast.best_score:.4f} key={beast.key}", flush=True)
