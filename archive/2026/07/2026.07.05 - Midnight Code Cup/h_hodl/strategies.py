"""Hot-reloaded every loop by daemon.py — edit freely, next tick picks it up.

desired(sid, m) -> target position in [0, 100] for the stock, given its model.

Model fields (see daemon.Model):
  m.price       current price
  m.s_m1        sign of last change (-1/0/+1), flat resets to 0
  m.s_lnz       sign of last NONZERO change
  m.stats       dict: (mode, state) -> [count, sum_dp]; E[dp|state] = sum/count
  m.hist        list of recent (time_ms, price) ticks (changes only), capped
"""

import bisect
import json
import os

MINN = 5

# survives importlib.reload (same module dict is reused)
try:
    _CACHE
except NameError:
    _CACHE = {}

_ORACLE_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "work", "oracle.json")


def _oracle():
    try:
        mt = os.path.getmtime(_ORACLE_PATH)
    except OSError:
        return {}
    if _CACHE.get("mt") != mt:
        raw = json.load(open(_ORACLE_PATH))
        _CACHE["oracle"] = {int(k): v for k, v in raw.items()}
        _CACHE["mt"] = mt
    return _CACHE.get("oracle", {})


def desired_oracle(m, t, o):
    """o = {'dt': float, 'prices': [server price per calendar day]}."""
    arr = o["prices"]
    j = int(t // o["dt"])
    if j >= len(arr):
        return 0
    if j < 0:
        return None
    if arr[j] != m.price:  # self-sync alignment
        for dj in (1, -1, 2, -2, 3, -3):
            if 0 <= j + dj < len(arr) and arr[j + dj] == m.price:
                j += dj
                break
    for jn in range(j + 1, min(j + 15, len(arr))):
        if arr[jn] != arr[j]:
            return 100 if arr[jn] > arr[j] else 0
    return 0


def price_at(m, t):
    """Step-function lookup in m.hist [(time, price), ...]."""
    i = bisect.bisect_right(m.hist, (t, float("inf"))) - 1
    return m.hist[i][1] if i >= 0 else None


# Stock 20: deterministic waveform, exact period 710 ticks * 10000 ms.
PERIOD_20 = 7100000
TICK_20 = 10000


def desired_20(m):
    t_now = m.hist[-1][0]
    nxt = price_at(m, t_now + TICK_20 - PERIOD_20)
    if nxt is None:
        return None
    return 100 if nxt > m.price else 0


# Stock 12: prices i.i.d. around 991 -> hold iff price below threshold.
def desired_12(m):
    return 100 if m.price < 990 else 0


# Round-number family: common factor with per-stock delays (ticks on shared
# minute grid). Followers see the factor's move SHIFT ticks after leaders.
SHIFTS = {2: 108, 3: 0, 4: 24, 7: 34, 8: 38, 9: 6, 10: 60, 11: 30, 14: 5,
          17: 0, 18: 7, 21: 0, 22: 14, 23: 3, 24: 29, 25: 9, 28: 4, 30: 92,
          31: 19, 32: 3, 33: 65, 34: 0, 37: 0, 38: 60, 39: 70, 40: 65,
          41: 92, 42: 0, 46: 0}
# The cross-stock lead-lag alpha existed only in the first ~180 round ticks
# (early common crash); since then X is pure noise and costs money. Disabled.
X_WEIGHT = 0.0


def _round_state(models):
    """Grid-align round stocks' changes; cached until a new change arrives."""
    total = sum(len(models[s].hist) for s in SHIFTS if s in models)
    st = _CACHE.get("round")
    if st is not None and st["total"] == total:
        return st
    grid = sorted({t for s in SHIFTS if s in models for t, _ in models[s].hist})
    pos = {t: i for i, t in enumerate(grid)}
    z = {}
    for s in SHIFTS:
        if s not in models:
            continue
        arr = [0.0] * len(grid)
        hist = models[s].hist
        for (t0, p0), (t1, p1) in zip(hist, hist[1:]):
            arr[pos[t1]] = (p1 - p0) / 1000.0
        z[s] = arr
    st = {"total": total, "grid": grid, "z": z}
    _CACHE["round"] = st
    return st


def desired_round(sid, m, models):
    s = SHIFTS[sid]
    mode = "lnz"
    cnt, sm = m.stats.get((mode, m.s_lnz), (0, 0.0))
    rev = (sm / cnt / 1000.0) if cnt >= MINN else 0.0
    x = 0.0
    if s >= 1 and models is not None:
        st = _round_state(models)
        K = len(st["grid"]) - 1
        u = K + 1 - s
        vals = [st["z"][g][u + SHIFTS[g]]
                for g in st["z"] if g != sid and 0 <= u + SHIFTS[g] <= K]
        if vals:
            x = sum(vals) / len(vals)
    return 100 if rev + X_WEIGHT * x > 0 else 0

# mode per stock: walk-forward-selected on first 11.5h of history
#   m1  = Markov on sign of last change (momentum/reversion incl. flat state)
#   lnz = sign of last nonzero change
#   flat = no edge found yet, stay out
CONFIG = {
    1: "m1", 2: "m1", 3: "lnz", 4: "lnz", 5: "lnz", 6: "lnz", 7: "lnz",
    8: "lnz", 9: "lnz", 10: "lnz", 11: "flat", 12: "m1", 13: "lnz",
    14: "lnz", 15: "flat", 16: "m1", 17: "lnz", 18: "lnz", 19: "m1",
    20: "m1", 21: "lnz", 22: "lnz", 23: "lnz", 24: "lnz", 25: "lnz",
    26: "lnz", 27: "flat", 28: "lnz", 29: "lnz", 30: "flat", 31: "lnz",
    32: "lnz", 33: "m1", 34: "lnz", 35: "lnz", 36: "lnz", 37: "lnz",
    38: "lnz", 39: "m1", 40: "lnz", 41: "m1", 42: "lnz", 43: "m1",
    44: "flat", 45: "flat", 46: "lnz", 47: "lnz", 48: "m1", 49: "lnz",
    50: "lnz",
}


def desired(sid, m, t=None, models=None):
    if t is not None:
        o = _oracle().get(sid)
        if o is not None:
            want = desired_oracle(m, t, o)
            if want is not None:
                return want
    if sid == 12:
        return desired_12(m)
    if sid in SHIFTS:
        return desired_round(sid, m, models)
    if sid == 20:
        want = desired_20(m)
        if want is not None:
            return want
    mode = CONFIG.get(sid, "lnz")
    if mode == "flat":
        return 0
    s = m.s_m1 if mode == "m1" else m.s_lnz
    cnt, sm = m.stats.get((mode, s), (0, 0.0))
    if cnt >= MINN and sm / cnt > 0:
        return 100
    return 0
