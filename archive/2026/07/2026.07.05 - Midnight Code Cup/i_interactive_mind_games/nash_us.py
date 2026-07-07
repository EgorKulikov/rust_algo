#!/usr/bin/env python3
"""Symmetric mixed Nash equilibrium for unique-smallest.

Model: nopp opponents' picks Poissonized — cnt_v ~ Poisson(nopp * p_v),
independent across values v. Then the expected payoff of playing value c is
exact and cheap:

  u(c) = sum_f P_c(f) * 0.8^(f/2) * prod_{v != c} h_v(f+1, c)
  h_v(F, c) = E[0.8^(cnt_v * [group v ranks before (F, c)])]
            = A_v(F) + P_v(F) * (0.8^F if v < c else 1) + T_v(F)

where a group ranks before ours iff (cnt_v, v) < (F, c) lexicographically,
A_v(F) = sum_{j<F} P_v(j)*0.8^j, T_v(F) = sum_{j>F} P_v(j), and rank within
our group of size f+1 averages to +f/2.

Replicator dynamics on these payoffs -> equilibrium; multinomial Monte Carlo
validates exploitability. Writes work/us_nash.json: {nopp: [p_1..p_M]}.
"""
import json
import math
from pathlib import Path

import numpy as np

M = 64          # values 1..M considered
J = 40          # Poisson count truncation
WORK = Path(__file__).resolve().parent / "work"


def payoffs(p, nopp):
    """Exact expected payoff of each pure value under the Poisson model."""
    lam = nopp * p                                   # (M,)
    j = np.arange(J)
    logpmf = -lam[:, None] + j[None, :] * np.log(np.maximum(lam[:, None], 1e-300)) - [
        math.lgamma(x + 1) for x in j]
    pmf = np.exp(logpmf)                             # (M, J) P_v(j)
    pmf[lam == 0, :] = 0.0
    pmf[lam == 0, 0] = 1.0
    w = pmf * (0.8 ** j)[None, :]                    # P_v(j)*0.8^j
    A = np.cumsum(w, axis=1) - w                     # A_v(F) = sum_{j<F}
    csum = np.cumsum(pmf, axis=1)
    T = 1.0 - csum                                   # T_v(F) = sum_{j>F}
    F = np.arange(1, J)                              # F = f+1 for f = 0..J-2
    less = A[:, F] + pmf[:, F] * (0.8 ** F)[None, :] + T[:, F]   # h if v < c
    geq = A[:, F] + pmf[:, F] + T[:, F]                          # h if v >= c
    Ll = np.log(np.maximum(less, 1e-300))            # (M, J-1)
    Lg = np.log(np.maximum(geq, 1e-300))
    CL = np.cumsum(Ll, axis=0)                       # sum over v <= c
    CG = np.cumsum(Lg, axis=0)
    # logprod[c] = sum_{v<c} Ll[v] + sum_{v>c} Lg[v]
    logprod = (CL - Ll) + (CG[-1][None, :] - CG)
    f = F - 1
    u = np.sum(pmf[:, f] * (0.8 ** (f / 2.0))[None, :] * np.exp(logprod), axis=1)
    return u


def solve(nopp, iters=60000, t0=50):
    """Damped fictitious play: p_{t+1} = (1-a_t) p_t + a_t e_{BR(p_t)}.
    Replicator dynamics fails here (it never explores off-support values, so
    'any unused large number' stays a profitable deviation)."""
    p = np.ones(M) / M
    for t in range(1, iters + 1):
        u = payoffs(p, nopp)
        br = int(np.argmax(u))
        a = 1.0 / (t + t0)
        p *= 1.0 - a
        p[br] += a
    u = payoffs(p, nopp)
    expl = float(u.max() - u @ p)
    return p, u, expl


def mc_check(p, nopp, samples=40000, seed=7):
    """Exploitability under the true multinomial-opponents model."""
    rng = np.random.default_rng(seed)
    n = max(1, int(round(nopp)))
    C = rng.multinomial(n, p, size=samples)          # (S, M)
    vals = np.arange(M)
    u = np.zeros(M)
    for c in range(M):
        f = C[:, c]
        newf = f + 1
        before = (C < newf[:, None]) | ((C == newf[:, None]) & (vals < c)[None, :])
        before[:, c] = False
        pos = 1 + np.sum(C * before, axis=1)
        u[c] = float(np.mean(0.8 ** (pos + f / 2.0 - 1.0)))
    return u


def main():
    out = {}
    for nopp in range(5, 41):
        p, u, expl = solve(nopp)
        umc = mc_check(p, nopp)
        expl_mc = float(umc.max() - umc @ p)
        supp = int(np.sum(p > 1e-3))
        top = ", ".join(f"{v+1}:{p[v]:.3f}" for v in range(M) if p[v] > 0.02)
        print(f"nopp={nopp:2d} support~{supp:2d} expl={expl:.4f} "
              f"expl_mc={expl_mc:.4f} eq_pay={float(u @ p):.4f}  {top}")
        out[str(nopp)] = [float(x) for x in p]
    (WORK / "us_nash.json").write_text(json.dumps(out))
    print("wrote", WORK / "us_nash.json")


if __name__ == "__main__":
    main()
