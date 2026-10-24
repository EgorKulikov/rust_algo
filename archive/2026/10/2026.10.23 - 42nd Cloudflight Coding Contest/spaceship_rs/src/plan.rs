//! Staged search for a valid plan. Each stage stops at the first plan that
//! fits the time limit; later stages are only reached when earlier ones fail.
//! 1. straight flight, optionally holding one axis back for a while
//! 2. one waypoint next to an asteroid, one delayed axis on one leg
//! 3. one waypoint, delays on both legs
//! 4. grid BFS around the asteroids, flown as axis-aligned legs
//! 5. two waypoints, one delayed axis on one leg

use crate::sim::{compose, simulate, Asteroid};
use std::collections::{BTreeSet, VecDeque};

pub struct Plan {
    pub time: usize,
    pub xp: Vec<i32>,
    pub yp: Vec<i32>,
}

pub struct Case {
    pub goal: (i32, i32),
    pub asteroids: Vec<Asteroid>,
    pub limit: usize,
}

fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// Candidate corners: cells at Chebyshev distance exactly 3 around each
/// asteroid, plus their projections onto the start and goal rows/columns,
/// sorted by detour length.
fn waypoints(case: &Case) -> Vec<(i32, i32)> {
    let (gx, gy) = case.goal;
    let mut wps = BTreeSet::new();
    for a in &case.asteroids {
        for ox in [-3, 0, 3] {
            for oy in [-3, 0, 3] {
                if ox != 0 || oy != 0 {
                    wps.insert((a.x + ox, a.y + oy));
                }
            }
        }
        for o in [-3, 3] {
            wps.insert((a.x + o, gy));
            wps.insert((a.x + o, 0));
            wps.insert((gx, a.y + o));
            wps.insert((0, a.y + o));
        }
    }
    wps.remove(&(0, 0));
    wps.remove(&case.goal);
    let safe = |w: &(i32, i32)| case.asteroids.iter().all(|a| (w.0 - a.x).abs() > 2 || (w.1 - a.y).abs() > 2);
    let mut v: Vec<(i32, i32)> = wps.into_iter().filter(safe).collect();
    v.sort_by_key(|&w| manhattan((0, 0), w) + manhattan(w, case.goal));
    v
}

fn legs_via(goal: (i32, i32), ws: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut pts = vec![(0, 0)];
    pts.extend_from_slice(ws);
    pts.push(goal);
    pts.windows(2).map(|w| (w[1].0 - w[0].0, w[1].1 - w[0].1)).collect()
}

fn attempt(case: &Case, legs: &[(i32, i32)], delays: &[(usize, usize)]) -> Option<Plan> {
    let (xp, yp) = compose(legs, delays);
    let v = simulate(&xp, &yp, case.goal, &case.asteroids, case.limit);
    v.ok.then_some(Plan { time: v.time, xp, yp })
}

const MAX_DELAY: usize = 60;

/// Delay vectors with exactly one delayed axis on leg `leg` of `n` legs.
fn one_axis_delays(n: usize, leg: usize, k: usize) -> [Vec<(usize, usize)>; 2] {
    let mut a = vec![(0, 0); n];
    let mut b = vec![(0, 0); n];
    a[leg] = (k, 0);
    b[leg] = (0, k);
    [a, b]
}

/// Shortest grid path keeping Chebyshev distance >= 3 from all asteroids,
/// compressed to its corner points.
fn bfs_corners(case: &Case) -> Option<Vec<(i32, i32)>> {
    let pts: Vec<(i32, i32)> = [(0, 0), case.goal].into_iter().chain(case.asteroids.iter().map(|a| (a.x, a.y))).collect();
    let (x0, x1) = (pts.iter().map(|p| p.0).min().unwrap() - 6, pts.iter().map(|p| p.0).max().unwrap() + 6);
    let (y0, y1) = (pts.iter().map(|p| p.1).min().unwrap() - 6, pts.iter().map(|p| p.1).max().unwrap() + 6);
    let w = (x1 - x0 + 1) as usize;
    let h = (y1 - y0 + 1) as usize;
    let idx = |x: i32, y: i32| ((y - y0) as usize) * w + (x - x0) as usize;
    let blocked = |x: i32, y: i32| case.asteroids.iter().any(|a| (x - a.x).abs() <= 2 && (y - a.y).abs() <= 2);
    let mut prev = vec![usize::MAX; w * h];
    let mut q = VecDeque::new();
    prev[idx(0, 0)] = idx(0, 0);
    q.push_back((0, 0));
    while let Some((x, y)) = q.pop_front() {
        if (x, y) == case.goal {
            break;
        }
        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if nx < x0 || nx > x1 || ny < y0 || ny > y1 || blocked(nx, ny) || prev[idx(nx, ny)] != usize::MAX {
                continue;
            }
            prev[idx(nx, ny)] = idx(x, y);
            q.push_back((nx, ny));
        }
    }
    if prev[idx(case.goal.0, case.goal.1)] == usize::MAX {
        return None;
    }
    let mut path = vec![case.goal];
    let mut cur = idx(case.goal.0, case.goal.1);
    while cur != idx(0, 0) {
        cur = prev[cur];
        path.push((x0 + (cur % w) as i32, y0 + (cur / w) as i32));
    }
    path.reverse();
    let mut corners = Vec::new();
    for i in 1..path.len() - 1 {
        let d1 = (path[i].0 - path[i - 1].0, path[i].1 - path[i - 1].1);
        let d2 = (path[i + 1].0 - path[i].0, path[i + 1].1 - path[i].1);
        if d1 != d2 {
            corners.push(path[i]);
        }
    }
    Some(corners)
}

/// Stage 1 only: straight flight with at most one held-back axis.
pub fn plan_direct(case: &Case) -> Option<Plan> {
    let goal = case.goal;
    for k in 0..=MAX_DELAY {
        for d in one_axis_delays(1, 0, k) {
            if let Some(p) = attempt(case, &[goal], &d) {
                return Some(p);
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn plan(case: &Case) -> Option<Plan> {
    let goal = case.goal;
    for k in 0..=MAX_DELAY {
        for d in one_axis_delays(1, 0, k) {
            if let Some(p) = attempt(case, &[goal], &d) {
                return Some(p);
            }
        }
    }
    let wps = waypoints(case);
    for &w in &wps {
        let legs = legs_via(goal, &[w]);
        for k in (0..=MAX_DELAY).step_by(2) {
            for leg in 0..2 {
                for d in one_axis_delays(2, leg, k) {
                    if let Some(p) = attempt(case, &legs, &d) {
                        return Some(p);
                    }
                }
            }
        }
    }
    for &w in &wps {
        let legs = legs_via(goal, &[w]);
        for k1 in (0..=MAX_DELAY).step_by(3) {
            for k2 in (0..=MAX_DELAY).step_by(3) {
                for d in [[(k1, 0), (k2, 0)], [(k1, 0), (0, k2)], [(0, k1), (k2, 0)], [(0, k1), (0, k2)]] {
                    if let Some(p) = attempt(case, &legs, &d) {
                        return Some(p);
                    }
                }
            }
        }
    }
    if let Some(corners) = bfs_corners(case) {
        let legs = legs_via(goal, &corners);
        if let Some(p) = attempt(case, &legs, &[]) {
            return Some(p);
        }
    }
    for &w in wps.iter().take(40) {
        for &w2 in wps.iter().take(40) {
            if w2 == w {
                continue;
            }
            let legs = legs_via(goal, &[w, w2]);
            for k in (0..=MAX_DELAY).step_by(4) {
                for leg in 0..3 {
                    for d in one_axis_delays(3, leg, k) {
                        if let Some(p) = attempt(case, &legs, &d) {
                            return Some(p);
                        }
                    }
                }
            }
        }
    }
    None
}
