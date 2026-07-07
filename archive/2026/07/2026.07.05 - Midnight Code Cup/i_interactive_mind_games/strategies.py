"""Per-round strategies. Hot-reloaded by daemon.py at every round boundary,
so edits here take effect on the next round without restarting the daemon.

Current tier: "reasonable participation" — feedback-aware heuristics with
randomization. Mixed-equilibrium work comes later.

ctx common fields:
  round    - current round number (1-based)
  config   - the game's /config response
  history  - list of /last-round responses (one per completed round we saw)
"""
import json
import random
import statistics
from pathlib import Path

_WORK = Path(__file__).resolve().parent / "work"

# ---------------------------------------------------------------- unique-smallest


def unique_smallest(ctx):
    """Empirical-count payoff argmax (the parking era, r~418+). The field
    stabilized into territories: single teams park on each small value
    (empirical P(free)=0 where Poisson said 0.37), so Poisson models
    overrate them and mixtures waste mass. This model uses each value's
    EMPIRICAL count distribution (20-round window, 10% Poisson smoothing)
    in the exact factorized payoff, then plays the argmax — deliberately
    deterministic: hammering the best semi-free value parks US there, and
    other teams' dodge models route around it. Refits every round; if our
    territory gets contested the argmax relocates."""
    hist = [fb for fb in ctx["history"] if fb.get("numbers")][-20:]
    if len(hist) < 8:
        return random.randint(12, 27)
    import numpy as np
    from math import lgamma
    M, J = 64, 8
    T = len(hist)
    cnt = np.zeros((M, J))
    lam = np.zeros(M)
    for fb in hist:
        nums = list(fb["numbers"])
        mine = (fb.get("your_move") or {}).get("number")
        if mine is not None and mine in nums:
            nums.remove(mine)
        cc = {}
        for v in nums:
            v = min(v, 60) if v > M else v
            cc[v] = cc.get(v, 0) + 1
        for v in range(1, M + 1):
            cnt[v - 1, min(cc.get(v, 0), J - 1)] += 1
            lam[v - 1] += cc.get(v, 0)
    emp = cnt / T
    lam /= T
    j = np.arange(J)
    pois = np.exp(-lam[:, None] + j[None, :] *
                  np.log(np.maximum(lam[:, None], 1e-300)) -
                  [lgamma(x + 1) for x in j])
    pois[lam == 0, :] = 0.0
    pois[lam == 0, 0] = 1.0
    pmf = 0.9 * emp + 0.1 * pois
    pmf /= pmf.sum(1, keepdims=True)
    # exact factorized payoff with arbitrary per-value count pmfs
    w = pmf * (0.8 ** j)[None, :]
    A = np.cumsum(w, 1) - w
    Tv = 1 - np.cumsum(pmf, 1)
    F = np.arange(1, J)
    less = A[:, F] + pmf[:, F] * (0.8 ** F)[None, :] + Tv[:, F]
    geq = A[:, F] + pmf[:, F] + Tv[:, F]
    Ll = np.log(np.maximum(less, 1e-300))
    Lg = np.log(np.maximum(geq, 1e-300))
    CL, CG = np.cumsum(Ll, 0), np.cumsum(Lg, 0)
    logprod = (CL - Ll) + (CG[-1][None, :] - CG)
    f = F - 1
    u = np.sum(pmf[:, f] * (0.8 ** (f / 2.0))[None, :] * np.exp(logprod), 1)
    return int(np.argmax(u[:45])) + 1


def _us_equilibrium(ctx):
    """Sample from the symmetric mixed Nash equilibrium (nash_us.py writes
    work/us_nash.json keyed by expected opponent count)."""
    hist = ctx["history"][-10:]
    sizes = [len(fb["numbers"]) for fb in hist if fb.get("numbers")]
    nopp = (statistics.median(sizes) - 1) if sizes else 13
    try:
        table = json.loads((_WORK / "us_nash.json").read_text())
    except FileNotFoundError:
        return random.randint(1, 12)
    key = min(table, key=lambda k: abs(int(k) - nopp))
    p = table[key]
    return random.choices(range(1, len(p) + 1), weights=p)[0]


# ------------------------------------------------------------------------ blotto


def blotto(ctx):
    """Sample the mixed-equilibrium shape mixture. The marginal-CDF exploit
    (_blotto_exploit below) earned well r110-280 but the field converged to
    the same spike meta: by r330 its predictions decorrelated from outcomes
    (corr 0.53 -> -0.09) and duels became mirror coin-flips. The wide
    equilibrium mixture randomizes spike levels and cannot be mirrored."""
    return _blotto_nash(ctx)


def _blotto_exploit(ctx):
    """(retired r331, kept for re-enable if the field de-spikifies)
    field_dominance -> marginal troop CDF; opponents iid; hill-climb
    P(win >= 3 of 5 fields)."""
    import numpy as np
    rows = [fb for fb in ctx["history"] if fb.get("your_move")][-30:]
    if len(rows) < 8:
        return _blotto_nash(ctx)

    pts_a, pts_q = [], []
    for fb in rows:
        ym = fb["your_move"]
        n = fb["teams_count"] - 1
        if n <= 0:
            continue
        for a, d in zip(ym["allocation"], ym["field_dominance"]):
            pts_a.append(a)
            pts_q.append(d / (2 * n))
    A, Q = np.array(pts_a), np.array(pts_q)
    xs = np.arange(101)
    q = np.full(101, np.nan)
    for x in xs:
        w = np.maximum(0, 4 - np.abs(A - x))
        if w.sum() > 0:
            q[x] = (w @ Q) / w.sum()
    valid = ~np.isnan(q)
    q = np.maximum.accumulate(np.interp(xs, xs[valid], q[valid]))

    def pwin(a):
        """P(win duel) + 0.5 P(draw) via per-field win/tie/loss trinomial;
        tie mass ~ CDF mass within +-1 troop (q folds ties in as halves, so
        treating q as pure win prob overvalues popular troop levels like 20)."""
        dp = {0: 1.0}
        for x in a:
            t = q[min(x + 1, 100)] - q[max(x - 1, 0)]
            pw = max(0.0, q[x] - 0.5 * t)
            pl = max(0.0, 1.0 - pw - t)
            ndp = {}
            for s, pr in dp.items():
                for ds, po in ((1, pw), (0, t), (-1, pl)):
                    ndp[s + ds] = ndp.get(s + ds, 0.0) + pr * po
            dp = ndp
        return sum(pr for s, pr in dp.items() if s > 0) + \
            0.5 * dp.get(0, 0.0)

    def climb(a):
        a = np.array(a, dtype=int)
        best = pwin(a)
        improved = True
        while improved:
            improved = False
            for i in range(5):
                for j in range(5):
                    if i == j:
                        continue
                    for d in (1, 2, 5, 10):
                        if a[i] < d:
                            continue
                        a[i] -= d
                        a[j] += d
                        v = pwin(a)
                        if v > best + 1e-12:
                            best, improved = v, True
                        else:
                            a[i] += d
                            a[j] -= d
        return best, tuple(sorted(a.tolist(), reverse=True))

    rng = np.random.default_rng()
    optima = {}
    for _ in range(10):
        seed = rng.multinomial(100, rng.dirichlet(np.ones(5)))
        v, a = climb(seed)
        optima[a] = v
    best = max(optima.values())
    near = [a for a, v in optima.items() if v > best - 0.004]
    alloc = list(random.choice(near))
    random.shuffle(alloc)
    return alloc


def _blotto_nash(ctx):
    """Sample a shape from the mixed equilibrium (blotto_nash.py writes
    work/blotto_nash.json keyed by opponent count) and play it under a
    uniformly random battlefield permutation."""
    try:
        table = json.loads((_WORK / "blotto_nash.json").read_text())
    except FileNotFoundError:
        return _blotto_fallback()
    hist = ctx["history"][-10:]
    sizes = [fb["teams_count"] for fb in hist if fb.get("teams_count")]
    nopp = (statistics.median(sizes) - 1) if sizes else 21
    key = min(table, key=lambda k: abs(int(k) - nopp))
    shapes, weights = zip(*table[key])
    # field meta check (r332-395, n=65): shapes with top spike >= 37 earned
    # 0.19 mean rank_score vs 0.10 below — the field's spikes sit at 33-36
    # and only 37+ clears them. Keep the mixture wide but drop low tops.
    # r988 check: top 37-38 earns 0.27-0.31, 41-42 only 0.03-0.06 (dominance
    # at 36-38 is already 0.91 — higher spikes starve the other fields)
    hi = [(s, w) for s, w in zip(shapes, weights) if 37 <= max(s) <= 40]
    if hi:
        shapes, weights = zip(*hi)
    alloc = list(random.choices(shapes, weights=weights)[0])
    random.shuffle(alloc)
    return alloc


def _blotto_fallback():
    """Each field ~uniform[0, 2*budget/fields], rescaled — the shape of the
    continuous-Blotto symmetric equilibrium."""
    while True:
        xs = [random.randint(0, 40) for _ in range(5)]
        s = sum(xs)
        if s < 20:
            continue
        scaled = [x * 100.0 / s for x in xs]
        alloc = [int(v) for v in scaled]
        rem = 100 - sum(alloc)
        order = sorted(range(5), key=lambda i: scaled[i] - alloc[i], reverse=True)
        for k in range(rem):
            alloc[order[k % 5]] += 1
        return alloc


# --------------------------------------------------------------------- door-rush


def _circ(a, b):
    d = abs(a - b)
    return min(d, 6 - d)


def _door_feats1(fb, i):
    """Phase-1 features: room parameters plus herd-attractiveness indicators
    (the field mobs the naive-EV top room: 5.1 rushers / 35% overflow vs
    2.6-2.9 / 12-15% for its rank-4..6 rooms; letting the logistic price
    that in backtested +12%)."""
    rooms = fb["rooms"]
    caps = [r["capacity"] for r in rooms]
    rews = [r["reward"] for r in rooms]
    rrank = sorted(range(6), key=lambda j: -rews[j]).index(i)
    naive = [rews[j] * fb["rush_bonus"] * min(caps[j] / 6, 1) for j in range(6)]
    ev_rank = sorted(range(6), key=lambda j: -naive[j]).index(i)
    return [1.0, caps[i] / 10, rews[i] / 200, rrank / 5, fb["rush_bonus"],
            sum(caps) / 40, caps[i] - sum(caps) / 6,
            1.0 if ev_rank == 0 else 0.0, 1.0 if ev_rank == 1 else 0.0]


def _door_feats2(fb, i):
    """Phase-2 features: adds the entrance-count margin (U-shaped risk:
    under-capacity rooms attract flex movers and overflow, crowded ones
    drain — learned, not assumed)."""
    rooms = fb["rooms"]
    caps = [r["capacity"] for r in rooms]
    rews = [r["reward"] for r in rooms]
    ec = fb["entrance_counts"]
    m = ec[i] - caps[i]
    rrank = sorted(range(6), key=lambda j: -rews[j]).index(i)
    return [1.0, m / 5, (m / 5) ** 2, ec[i] / 10, caps[i] / 10,
            rews[i] / 200, rrank / 5]


def _door_target(fb, i):
    """1 if room i would have stayed within capacity with us in it."""
    ym = fb.get("your_move") or {}
    add = 0 if ym.get("final_room") == i else 1
    return 1.0 if fb["final_counts"][i] + add <= fb["rooms"][i]["capacity"] else 0.0


def _door_models(history):
    """Train P(safe) logistics on all finalized rounds; None if too few."""
    import numpy as np
    # recent window only: the field's herding habits drift (r~600 rush mobs);
    # full-history training dilutes the current regime (backtest 140 vs 130)
    train = [fb for fb in history
             if fb.get("final_counts") and fb.get("entrance_counts")][-75:]
    if len(train) < 30:
        return None
    y = np.array([_door_target(fb, i) for fb in train for i in range(6)])

    def fit(X):
        X = np.array(X)
        w = np.zeros(X.shape[1])
        for _ in range(3000):
            p = 1 / (1 + np.exp(-X @ w))
            w -= 0.5 * (X.T @ (p - y) / len(y) + 1e-2 * w)
        return w

    w1 = fit([_door_feats1(fb, i) for fb in train for i in range(6)])
    w2 = fit([_door_feats2(fb, i) for fb in train for i in range(6)])
    return w1, w2


def _sigmoid_dot(w, x):
    import math
    return 1 / (1 + math.exp(-sum(a * b for a, b in zip(w, x))))


def door_phase1(ctx):
    """Rush the argmax of reward * rush_bonus * P(safe | room params) when it
    beats a flex-value proxy; backtested ~107/round vs 63 for the old flex
    heuristic. Falls back to the heuristic until enough training rounds."""
    rooms = ctx["rooms"]
    models = _door_models(ctx["history"])
    if models is None:
        weights = [max(1.0, r["reward"]) * r["capacity"] for r in rooms]
        return random.choices(range(6), weights=weights)[0], "flex"
    w1, _ = models
    fb = {"rooms": rooms, "rush_bonus": ctx["rush_bonus"]}
    psafe = [_sigmoid_dot(w1, _door_feats1(fb, i)) for i in range(6)]
    # congestion market reached rough equilibrium (r~810: survival flat
    # 0.68-0.78 across capacity ranks) — fixed lanes get arbitraged within
    # ~50 rounds (razor mids died r770, then the field followed us into big
    # rooms). Mixed play: sample among rooms within 85% of max EV
    # (backtest 134.9/round, 20% deaths vs 100.9/32% for top-2-cap lane).
    rush_ev = [rooms[i]["reward"] * ctx["rush_bonus"] * psafe[i] for i in range(6)]
    # endgame check r1063: small rooms killed us at 62% vs field 27% —
    # cap >= 6 floor backtests 116.6/round @ 16% deaths vs 100.0 @ 24%
    caps = [r["capacity"] for r in rooms]
    cand = [i for i in range(6) if caps[i] >= 6] or list(range(6))
    mx = max(rush_ev[i] for i in cand)
    near = [i for i in cand if rush_ev[i] >= 0.85 * mx]
    return int(random.choice(near)), "rush"


def door_phase2(ctx):
    """Pick the final room by reward * 0.75^dist * learned P(safe | entrance
    counts). Only called when phase 1 chose flex."""
    rooms = ctx["rooms"]
    counts = ctx["counts"]
    entrance = ctx["entrance"]
    models = _door_models(ctx["history"])
    if models is None:
        def safety(i):
            margin = rooms[i]["capacity"] - counts[i]
            return {2: 1.0, 1: 0.85, 0: 0.45}.get(min(margin, 2), 0.05)
    else:
        _, w2 = models
        fb = {"rooms": rooms, "entrance_counts": counts}

        def safety(i):
            return _sigmoid_dot(w2, _door_feats2(fb, i))

    def value(i):
        return rooms[i]["reward"] * (0.75 ** _circ(entrance, i)) * safety(i)

    return max(range(6), key=value)
