//! Mirror of the contest sandbox simulation.
//!
//! A pace `p` lasts `|p|` time units and moves the ship one grid unit in the
//! sign direction at the end of it; pace 0 lasts one unit and does not move.
//! Paces can only change by one step along `-1..-5, 0, 5..1`. After every
//! time unit the ship must keep Chebyshev distance >= 3 from every asteroid.

pub const MAX_PACE: i32 = 5;

#[derive(Clone, Copy, Debug)]
pub struct Asteroid {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

impl Asteroid {
    pub fn at(&self, t: i32) -> (i32, i32) {
        // The sandbox uses truncation toward zero of t / v.
        let shift = |v: i32| if v == 0 { 0 } else { t / v };
        (self.x + shift(self.vx), self.y + shift(self.vy))
    }
}

/// One entry per time unit.
pub fn expand(paces: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for &p in paces {
        for _ in 0..p.abs().max(1) {
            out.push(p);
        }
    }
    out
}

pub fn valid_transitions(paces: &[i32]) -> bool {
    if paces.first() != Some(&0) || paces.last() != Some(&0) {
        return false;
    }
    let mut prev: i32 = 0;
    for &p in paces {
        let ok = if prev == 0 {
            p == 0 || p.abs() == MAX_PACE
        } else if p == 0 {
            prev.abs() == MAX_PACE
        } else {
            p.signum() == prev.signum() && (p - prev).abs() <= 1
        };
        if !ok {
            return false;
        }
        prev = p;
    }
    true
}

pub struct Verdict {
    pub ok: bool,
    pub time: usize,
    pub reason: String,
}

pub fn simulate(xp: &[i32], yp: &[i32], goal: (i32, i32), asteroids: &[Asteroid], limit: usize) -> Verdict {
    if !valid_transitions(xp) || !valid_transitions(yp) {
        return Verdict { ok: false, time: 0, reason: "bad transitions".into() };
    }
    let (mut ex, mut ey) = (expand(xp), expand(yp));
    let n = ex.len().max(ey.len());
    if n > limit {
        return Verdict { ok: false, time: n, reason: format!("too slow {} > {}", n, limit) };
    }
    ex.resize(n, 0);
    ey.resize(n, 0);
    let (mut x, mut y, mut tx, mut ty) = (0, 0, 0, 0);
    for t in 0..n {
        let (vx, vy) = (ex[t], ey[t]);
        if vx != 0 {
            tx += 1;
            if tx >= vx.abs() {
                tx = 0;
                x += vx.signum();
            }
        }
        if vy != 0 {
            ty += 1;
            if ty >= vy.abs() {
                ty = 0;
                y += vy.signum();
            }
        }
        for a in asteroids {
            let (px, py) = a.at(t as i32 + 1);
            if (x - px).abs() <= 2 && (y - py).abs() <= 2 {
                return Verdict { ok: false, time: n, reason: format!("collision at step {} at ({},{})", t + 1, x, y) };
            }
        }
    }
    if (x, y) != goal {
        return Verdict { ok: false, time: n, reason: format!("missed goal, at ({},{})", x, y) };
    }
    Verdict { ok: true, time: n, reason: "ok".into() }
}

/// Fastest pace list covering `d` units in one direction: enters and leaves at
/// pace 5, ramps towards 1 in the middle. DP over (units done, |pace|).
pub fn best_sequence(d: i32) -> Vec<i32> {
    let d = d.unsigned_abs() as usize;
    if d == 0 {
        return Vec::new();
    }
    const INF: i32 = i32::MAX / 2;
    let mut dp = vec![[INF; 6]; d + 1];
    let mut prev = vec![[0usize; 6]; d + 1];
    dp[1][5] = 5;
    for i in 1..d {
        for k in 1..=5usize {
            if dp[i][k] == INF {
                continue;
            }
            for nk in [k - 1, k, k + 1] {
                if (1..=5).contains(&nk) && dp[i][k] + (nk as i32) < dp[i + 1][nk] {
                    dp[i + 1][nk] = dp[i][k] + nk as i32;
                    prev[i + 1][nk] = k;
                }
            }
        }
    }
    let mut seq = Vec::with_capacity(d);
    let (mut i, mut k) = (d, 5usize);
    while i >= 1 {
        seq.push(k as i32);
        k = prev[i][k];
        i -= 1;
    }
    seq.reverse();
    seq
}

/// Signed version of `best_sequence`.
pub fn seq(v: i32) -> Vec<i32> {
    let s = best_sequence(v);
    if v < 0 {
        s.into_iter().map(|p| -p).collect()
    } else {
        s
    }
}

/// Builds the two pace lists for a list of legs `(dx, dy)`. Both axes start
/// each leg at the same time; `delays[i] = (zeros before x, zeros before y)`.
pub fn compose(legs: &[(i32, i32)], delays: &[(usize, usize)]) -> (Vec<i32>, Vec<i32>) {
    let (mut xp, mut yp) = (vec![0], vec![0]);
    let (mut tx, mut ty) = (1usize, 1usize);
    for (i, &(dx, dy)) in legs.iter().enumerate() {
        let (ddx, ddy) = delays.get(i).copied().unwrap_or((0, 0));
        let t0 = tx.max(ty);
        xp.extend(std::iter::repeat(0).take(t0 - tx + ddx));
        yp.extend(std::iter::repeat(0).take(t0 - ty + ddy));
        let (sx, sy) = (seq(dx), seq(dy));
        tx = t0 + ddx + sx.iter().map(|p| p.unsigned_abs() as usize).sum::<usize>() + 1;
        ty = t0 + ddy + sy.iter().map(|p| p.unsigned_abs() as usize).sum::<usize>() + 1;
        xp.extend(sx);
        yp.extend(sy);
        xp.push(0);
        yp.push(0);
    }
    (xp, yp)
}
