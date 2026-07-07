#!/usr/bin/env python3
"""Approximate symmetric mixed Nash equilibrium for Blotto (rank payoff).

Strategy space: sorted-desc shapes (partitions of 100 into 5 parts), each
played with a uniform random battlefield permutation — battlefields are
symmetric, so a permutation-invariant equilibrium exists and shapes are WLOG.

Payoff: full round-robin rank among nopp+1 players, 0.8^(rank-1); ties get
average rank. Opponents drawn iid from the population mixture (our own
permutation is then irrelevant, so candidates stay sorted).

Solver: damped fictitious play; the best response each step is the argmax of
Monte-Carlo payoff over a candidate pool = current support + fresh random
shapes + mutations of the support (replicator-style local search would miss
off-support deviations, same lesson as nash_us.py).

Writes work/blotto_nash.json: {nopp: [[shape(5 desc), weight], ...]}.
"""
import json
import sys
from pathlib import Path

import numpy as np

WORK = Path(__file__).resolve().parent / "work"
rng = np.random.default_rng(12345)


def canon(a):
    return tuple(sorted((int(x) for x in a), reverse=True))


def random_shapes(k):
    """Mix of flat / medium / spiky random compositions."""
    out = []
    for alpha in (0.35, 1.0, 3.0):
        w = rng.dirichlet(np.ones(5) * alpha, size=k // 3 + 1)
        for row in w:
            a = np.floor(row * 100).astype(int)
            a[np.argsort(row * 100 - a)[::-1][: 100 - a.sum()]] += 1
            out.append(canon(a))
    return out[:k]


def mutations(shapes, k):
    out = []
    for _ in range(k):
        s = list(shapes[rng.integers(len(shapes))])
        for _ in range(rng.integers(1, 4)):
            i, j = rng.integers(5), rng.integers(5)
            d = int(min(s[i], rng.integers(1, 12)))
            s[i] -= d
            s[j] += d
        out.append(canon(s))
    return out


def payoffs_mc(cands, shapes, weights, nopp, samples, chunk=50):
    """cands (K,5) int; population = shapes (m,5) with weights. Returns
    MC-estimated expected 0.8^(rank-1) for each candidate."""
    K = len(cands)
    C = np.asarray(cands, dtype=np.int16)                    # (K, 5)
    S_arr = np.asarray(shapes, dtype=np.int16)               # (m, 5)
    total = np.zeros(K)
    done = 0
    while done < samples:
        s = min(chunk, samples - done)
        done += s
        idx = rng.choice(len(S_arr), size=(s, nopp), p=weights)
        O = S_arr[idx]                                       # (s, nopp, 5)
        perm = np.argsort(rng.random((s, nopp, 5)), axis=2)
        O = np.take_along_axis(O, perm, axis=2)
        # round robin among opponents
        gt = (O[:, :, None, :] > O[:, None, :, :]).sum(3)    # (s, nopp, nopp)
        beat = gt > gt.transpose(0, 2, 1)
        tie = gt == gt.transpose(0, 2, 1)
        s_base = beat.sum(2) + 0.5 * tie.sum(2) - 0.5        # minus self-tie
        # candidates vs opponents
        fwc = (C[None, None, :, :] > O[:, :, None, :]).sum(3)  # (s, nopp, K)
        fwo = (C[None, None, :, :] < O[:, :, None, :]).sum(3)
        beat_c = fwc > fwo
        tie_c = fwc == fwo
        score_c = beat_c.sum(1) + 0.5 * tie_c.sum(1)         # (s, K)
        opp_tot = s_base[:, :, None] + (fwo > fwc) + 0.5 * tie_c  # (s, nopp, K)
        better = (opp_tot > score_c[:, None, :] + 1e-9).sum(1)
        equal = (np.abs(opp_tot - score_c[:, None, :]) <= 1e-9).sum(1)
        rank = 1.0 + better + 0.5 * equal
        total += (0.8 ** (rank - 1.0)).sum(0)
    return total / done


def solve(nopp, iters=1500, t0=25, pool_fresh=400, pool_mut=200):
    mix = {canon([20, 20, 20, 20, 20]): 1.0}
    for t in range(1, iters + 1):
        samples = 200 if t < 500 else (400 if t < 1000 else 800)
        support = list(mix.keys())
        pool = list(dict.fromkeys(
            support + random_shapes(pool_fresh) + mutations(support, pool_mut)))
        w = np.array([mix[s] for s in support])
        w = w / w.sum()
        u = payoffs_mc(pool, support, w, nopp, samples)
        br = pool[int(np.argmax(u))]
        a = 1.0 / (t + t0)
        for s in mix:
            mix[s] *= 1.0 - a
        mix[br] = mix.get(br, 0.0) + a
        mix = {s: x for s, x in mix.items() if x > 5e-5}
        if t % 100 == 0:
            avg_u = float(np.array([u[pool.index(s)] for s in support]) @ w)
            print(f"  nopp={nopp} t={t} support={len(mix)} "
                  f"br_u={float(u.max()):.4f} mix_u={avg_u:.4f} "
                  f"br={list(br)}", flush=True)
    return mix


def validate(mix, nopp, samples=1500, sweep=6000):
    support = list(mix.keys())
    w = np.array([mix[s] for s in support])
    w = w / w.sum()
    pool = list(dict.fromkeys(
        support + random_shapes(sweep) + mutations(support, sweep // 3)))
    u = payoffs_mc(pool, support, w, nopp, samples, chunk=25)
    mix_u = float(np.array([u[pool.index(s)] for s in support]) @ w)
    br_i = int(np.argmax(u))
    print(f"  validate nopp={nopp}: mix_u={mix_u:.4f} "
          f"br_u={float(u[br_i]):.4f} expl={float(u[br_i]) - mix_u:.4f} "
          f"br={pool[br_i]}", flush=True)
    return mix_u, float(u[br_i])


def main():
    nopps = [int(x) for x in sys.argv[1:]] or [13, 17, 21, 25]
    path = WORK / "blotto_nash.json"
    out = json.loads(path.read_text()) if path.exists() else {}
    for nopp in nopps:
        print(f"solving nopp={nopp}", flush=True)
        mix = solve(nopp)
        validate(mix, nopp)
        top = sorted(mix.items(), key=lambda kv: -kv[1])
        print("  top:", [(list(s), round(x, 3)) for s, x in top[:8]], flush=True)
        out[str(nopp)] = [[list(s), float(x)] for s, x in top]
        path.write_text(json.dumps(out))
    print("wrote", path)


if __name__ == "__main__":
    main()
