"""Mirror of the level sandbox simulation: paces -> per-time-unit velocity, grid moves on tick, collision = Chebyshev <= 2."""
from solve3 import best_sequence

def expand(paces):
    out = []
    for p in paces:
        out += [p] * max(1, abs(p))
    return out

def valid_transitions(paces):
    prev = 0
    if not paces or paces[0] != 0 or paces[-1] != 0:
        return False
    for p in paces:
        if abs(prev - p) > 1 and not ({prev, p} <= {0, 5} or {prev, p} <= {0, -5}):
            return False
        if p == 0 and prev not in (0, 5, -5):
            return False
        prev = p
    return True

def asteroid_pos(a, t):
    (ax, ay), (vx, vy) = a
    def shift(v):
        if v == 0: return 0
        q = t / v
        return int(q)  # truncation toward zero, like parseInt(toPrecision())
    return ax + shift(vx), ay + shift(vy)

def simulate(xp, yp, goal, asteroids, limit):
    """asteroids: list of ((x, y), (vx, vy)). Returns (ok, message, time)."""
    if not valid_transitions(xp) or not valid_transitions(yp):
        return False, 'bad transitions', 0
    ex, ey = expand(xp), expand(yp)
    n = max(len(ex), len(ey))
    if n > limit:
        return False, f'too slow {n} > {limit}', n
    ex += [0] * (n - len(ex)); ey += [0] * (n - len(ey))
    x = y = 0; tx = ty = 0
    for t in range(n):
        vx, vy = ex[t], ey[t]
        if vx:
            tx += 1
            if tx >= abs(vx): tx = 0; x += 1 if vx > 0 else -1
        if vy:
            ty += 1
            if ty >= abs(vy): ty = 0; y += 1 if vy > 0 else -1
        for a in asteroids:
            px, py = asteroid_pos(a, t + 1)
            if abs(x - px) <= 2 and abs(y - py) <= 2:
                return False, f'collision at step {t + 1} at ({x},{y})', n
    if (x, y) != tuple(goal):
        return False, f'missed goal: at ({x},{y})', n
    return True, 'ok', n

def seq(v):
    s = best_sequence(abs(v))
    return [-p for p in s] if v < 0 else s

def compose(legs, delays=None):
    """legs: list of (dx, dy); both axes start each leg together. delays: per-leg (dx_delay, dy_delay) zeros."""
    xp, yp = [0], [0]
    tx = ty = 1
    for i, (dx, dy) in enumerate(legs):
        d = delays[i] if delays else (0, 0)
        t0 = max(tx, ty)
        xp += [0] * (t0 - tx + d[0]); yp += [0] * (t0 - ty + d[1])
        sx, sy = seq(dx), seq(dy)
        xp += sx; yp += sy
        tx = t0 + d[0] + sum(abs(p) for p in sx); ty = t0 + d[1] + sum(abs(p) for p in sy)
        xp.append(0); yp.append(0); tx += 1; ty += 1
    return xp, yp

def waypoints(goal, asteroids, extra=()):
    gx, gy = goal
    wps = set(extra)
    for (ax, ay), _ in asteroids:
        for ox in (-3, 0, 3):
            for oy in (-3, 0, 3):
                if ox or oy:
                    wps.add((ax + ox, ay + oy))
        for ox in (-3, 3):
            wps.add((ax + ox, gy)); wps.add((ax + ox, 0))
        for oy in (-3, 3):
            wps.add((gx, ay + oy)); wps.add((0, ay + oy))
    wps.discard((0, 0)); wps.discard(goal)
    return sorted(wps)

def plan(goal, asteroids, limit, extra_waypoints=()):
    """Fastest valid plan found in staged searches: direct with one delayed axis,
    one waypoint with a delayed axis on the second leg, then deeper combinations."""
    gx, gy = goal
    wps = waypoints(goal, asteroids, extra_waypoints)
    def legs_via(ws):
        pts = [(0, 0)] + list(ws) + [goal]
        return [(pts[i + 1][0] - pts[i][0], pts[i + 1][1] - pts[i][1]) for i in range(len(pts) - 1)]
    def stage(cands):
        best = None
        for legs, delays in cands:
            xp, yp = compose(legs, delays)
            ok, msg, t = simulate(xp, yp, goal, asteroids, limit)
            if ok and (best is None or t < best[0]):
                best = (t, xp, yp)
        return best
    direct = [([(gx, gy)], None)] + [([(gx, gy)], [d]) for k in range(1, 46) for d in ((k, 0), (0, k))]
    best = stage(direct)
    if best: return best
    one = []
    for w in wps:
        legs = legs_via([w])
        one.append((legs, None))
        for k in range(2, 46, 2):
            one.append((legs, [(0, 0), (k, 0)])); one.append((legs, [(0, 0), (0, k)]))
    best = stage(one)
    if best: return best
    deep = []
    for w in wps:
        legs = legs_via([w])
        for k1 in range(0, 46, 4):
            for k2 in range(0, 46, 4):
                for d1 in ((k1, 0), (0, k1)):
                    for d2 in ((k2, 0), (0, k2)):
                        deep.append((legs, [d1, d2]))
    best = stage(deep)
    if best: return best
    two = [(legs_via([w, w2]), None) for w in wps for w2 in wps if w != w2]
    two += [(legs_via([w, w2]), [(0, 0), (0, 0), d]) for w in wps for w2 in wps if w != w2 for k in range(4, 46, 4) for d in ((k, 0), (0, k))]
    return stage(two)
