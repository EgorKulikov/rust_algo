"""Simulated annealing for a beast (tuned for B, which greedy gets stuck on).

B is smooth and blackness-dominated: greedy coordinate-ascent parks at a local
optimum (~90.3) because any single white flip costs blackness up front, even if
the right white *structure* would score higher. SA accepts uphill moves always
and downhill moves with probability exp(delta / T), so it can pay the short-term
blackness cost to reach a better basin, then anneal down to refine.

Move: XOR a random block (size annealed large->small) at a random position.
Keeps a separate best image; periodic reheat to escape if stagnating.
Deterministic RNG (no wall-clock) so runs are reproducible/resumable-ish.
"""
import sys
import math
import numpy as np
import mcc

N = mcc.N


def anneal(beast, iters=200000, t_hi=0.6, t_lo=0.02, seed=0,
           block_sizes=(1, 1, 2, 2, 4, 8, 16), reheat_every=1500):
    rng = np.random.default_rng(seed)
    cur = beast.best_arr.copy() if beast.best_arr is not None else np.zeros((N, N), bool)
    cur_score, _ = beast.query(cur, tag="sa:seed")
    best = cur.copy()
    best_score = cur_score
    since_best = 0
    for it in range(iters):
        if beast.key:
            return best
        frac = it / iters
        T = t_hi * (t_lo / t_hi) ** frac
        # bias toward smaller blocks as we cool
        bmax = max(1, int(round(16 * (1 - frac) ** 2)))
        cand = [b for b in block_sizes if b <= max(2, bmax)] or [1]
        s = int(rng.choice(cand))
        y = int(rng.integers(0, N - s + 1))
        x = int(rng.integers(0, N - s + 1))
        trial = cur.copy()
        trial[y:y + s, x:x + s] ^= True
        sc, _ = beast.query(trial, tag=f"sa:it{it}:s{s}")
        d = sc - cur_score
        if d >= 0 or rng.random() < math.exp(d / max(T, 1e-6)):
            cur = trial
            cur_score = sc
        if sc > best_score:
            best_score = sc
            best = trial.copy()
            since_best = 0
        else:
            since_best += 1
        if it % 100 == 0:
            print(f"[{beast.name}] sa it={it} T={T:.3f} s={s} cur={cur_score:.3f} "
                  f"best={best_score:.4f}", flush=True)
        if since_best >= reheat_every:
            # reheat: jump back to best and raise effective temperature
            cur = best.copy()
            cur_score = best_score
            since_best = 0
            t_hi = min(t_hi * 1.3, 5.0)
            print(f"[{beast.name}] sa reheat -> t_hi={t_hi:.2f}", flush=True)
    return best


if __name__ == "__main__":
    name = sys.argv[1] if len(sys.argv) > 1 else "b"
    beast = mcc.Beast(name)
    print(f"[{name}] SA start best={beast.best_score:.4f} key={beast.key}", flush=True)
    anneal(beast)
    print(f"[{name}] SA end best={beast.best_score:.4f} key={beast.key}", flush=True)
