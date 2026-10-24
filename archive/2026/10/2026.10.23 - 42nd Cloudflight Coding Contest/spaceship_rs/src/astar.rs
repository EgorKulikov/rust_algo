//! Time-expanded A* that mirrors the sandbox exactly, run inside a corridor
//! around a grid-BFS path so that the state space stays small.
//!
//! State at the start of a time unit: cell, and per axis the current pace and
//! how many units of that entry are still to run (0 = choose the next entry).
//! A chosen entry runs `|p|` units (1 for pace 0) and moves one cell at its
//! end. Cost is elapsed time; the heuristic is the obstacle-free remaining
//! time of the slower axis from a small DP table.

use crate::sim::Asteroid;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};

/// Per-axis phase: (pace, units left). 42 combinations.
const PHASES: usize = 42;

fn phase_index(p: i8, a: u8) -> usize {
    // p = 0: a in {0,1}; |p| = k: a in 0..=k
    let base = match p.abs() {
        0 => 0,
        1 => 2,
        2 => 6,
        3 => 12,
        4 => 20,
        5 => 30,
        _ => unreachable!(),
    };
    let per = p.unsigned_abs() as usize + 1;
    base + if p < 0 { per } else { 0 } + a as usize
}

fn phase_decode(i: usize) -> (i8, u8) {
    let (mut p, mut start) = (0i8, 0usize);
    for k in 0..=5i8 {
        let per = k as usize + 1;
        let n = if k == 0 { 2 } else { 2 * per };
        if i < start + n {
            let off = i - start;
            if k == 0 {
                return (0, off as u8);
            }
            p = if off < per { k } else { -k };
            return (p, (off % per) as u8);
        }
        start += n;
    }
    let _ = p;
    unreachable!()
}

fn next_paces(p: i8) -> Vec<i8> {
    match p {
        0 => vec![0, 5, -5],
        5 => vec![5, 4, 0],
        -5 => vec![-5, -4, 0],
        1 => vec![1, 2],
        -1 => vec![-1, -2],
        _ => vec![p - 1, p, p + 1],
    }
}

/// `table[d][p]`: minimal time to cover `d` more cells when the last finished
/// entry had magnitude `p` (towards the goal, or 0), ending with a 0 entry.
fn axis_table(max_d: usize) -> Vec<[u32; 6]> {
    const INF: u32 = u32::MAX / 4;
    let mut t = vec![[INF; 6]; max_d + 1];
    t[0][0] = 1;
    t[0][5] = 1;
    for d in 1..=max_d {
        for _ in 0..6 {
            for p in 0..=5usize {
                let cands: Vec<usize> = match p {
                    0 => vec![5],
                    5 => vec![5, 4],
                    1 => vec![1, 2],
                    _ => vec![p - 1, p, p + 1],
                };
                for q in cands {
                    let c = q as u32 + t[d - 1][q];
                    if c < t[d][p] {
                        t[d][p] = c;
                    }
                }
            }
        }
    }
    t
}

pub struct Field {
    x0: i32,
    y0: i32,
    w: usize,
    h: usize,
    blocked: Vec<bool>,
}

impl Field {
    pub fn new(goal: (i32, i32), asteroids: &[Asteroid], margin: i32) -> Field {
        let xs = [0, goal.0].into_iter().chain(asteroids.iter().map(|a| a.x));
        let ys = [0, goal.1].into_iter().chain(asteroids.iter().map(|a| a.y));
        let (x0, x1) = (xs.clone().min().unwrap() - margin, xs.max().unwrap() + margin);
        let (y0, y1) = (ys.clone().min().unwrap() - margin, ys.max().unwrap() + margin);
        let w = (x1 - x0 + 1) as usize;
        let h = (y1 - y0 + 1) as usize;
        let mut blocked = vec![false; w * h];
        for a in asteroids {
            for dx in -2..=2 {
                for dy in -2..=2 {
                    let (x, y) = (a.x + dx, a.y + dy);
                    if x >= x0 && x <= x1 && y >= y0 && y <= y1 {
                        blocked[((y - y0) as usize) * w + (x - x0) as usize] = true;
                    }
                }
            }
        }
        Field { x0, y0, w, h, blocked }
    }

    fn inside(&self, x: i32, y: i32) -> bool {
        x >= self.x0 && y >= self.y0 && ((x - self.x0) as usize) < self.w && ((y - self.y0) as usize) < self.h
    }

    pub fn free(&self, x: i32, y: i32) -> bool {
        self.inside(x, y) && !self.blocked[((y - self.y0) as usize) * self.w + (x - self.x0) as usize]
    }

    /// 4-connected shortest path over free cells from the origin to `goal`.
    fn bfs_path(&self, goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        let idx = |x: i32, y: i32| ((y - self.y0) as usize) * self.w + (x - self.x0) as usize;
        let mut prev = vec![u32::MAX; self.w * self.h];
        let mut q = VecDeque::new();
        prev[idx(0, 0)] = idx(0, 0) as u32;
        q.push_back((0, 0));
        while let Some((x, y)) = q.pop_front() {
            if (x, y) == goal {
                let mut path = vec![goal];
                let mut cur = idx(goal.0, goal.1);
                while cur != idx(0, 0) {
                    cur = prev[cur] as usize;
                    path.push((self.x0 + (cur % self.w) as i32, self.y0 + (cur / self.w) as i32));
                }
                path.reverse();
                return Some(path);
            }
            for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if self.free(nx, ny) && prev[idx(nx, ny)] == u32::MAX {
                    prev[idx(nx, ny)] = idx(x, y) as u32;
                    q.push_back((nx, ny));
                }
            }
        }
        None
    }
}

pub struct Found {
    pub xp: Vec<i32>,
    pub yp: Vec<i32>,
    pub time: usize,
}

/// Tries corridors of increasing width around the BFS path.
pub fn search(goal: (i32, i32), field: &Field, limit: usize) -> Option<Found> {
    let path = field.bfs_path(goal)?;
    for radius in [4, 8, 16, 32, 1 << 20] {
        let t = std::time::Instant::now();
        let res = search_in_corridor(goal, field, limit, &path, radius);
        eprintln!("  radius {:>7}: {} in {:.2}s", radius, if res.is_some() { "found" } else { "exhausted" }, t.elapsed().as_secs_f64());
        if res.is_some() {
            return res;
        }
    }
    None
}

fn search_in_corridor(goal: (i32, i32), field: &Field, limit: usize, path: &[(i32, i32)], radius: i32) -> Option<Found> {
    // corridor cells
    let mut cells: Vec<(i32, i32)> = Vec::new();
    let mut cell_id: HashMap<(i32, i32), u32> = HashMap::new();
    for &(px, py) in path {
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                let c = (px + dx, py + dy);
                if field.free(c.0, c.1) && !cell_id.contains_key(&c) {
                    cell_id.insert(c, cells.len() as u32);
                    cells.push(c);
                }
            }
        }
    }
    let n_states = cells.len() * PHASES * PHASES;
    let mut best = vec![u32::MAX; n_states];
    let mut parent = vec![u32::MAX; n_states];
    let mut entry_x = vec![i8::MIN; n_states];
    let mut entry_y = vec![i8::MIN; n_states];
    let encode = |cell: u32, px: i8, ax: u8, py: i8, ay: u8| -> usize {
        cell as usize * PHASES * PHASES + phase_index(px, ax) * PHASES + phase_index(py, ay)
    };
    let max_d = field.w + field.h + 10;
    let table = axis_table(max_d);
    let per_axis = |pos: i32, g: i32, p: i8, a: u8| -> u32 {
        let d = g - pos;
        let dir = d.signum();
        let mut dist = d.abs();
        if a > 0 && p != 0 {
            if (p as i32).signum() == dir { dist -= 1 } else { dist += 1 }
        }
        let dist = dist.max(0) as usize;
        let m = if p != 0 && (p as i32).signum() != dir { 0 } else { p.unsigned_abs() as usize };
        let col = if dist == 0 { if m == 5 { 5 } else { 0 } } else { m };
        let mut t = table[dist.min(max_d)][col];
        if t >= u32::MAX / 4 {
            // cannot stop in time from this pace: must overshoot and come back,
            // which costs at least as much as finishing from a standstill
            t = table[dist.min(max_d)][0];
        }
        a as u32 + t
    };
    let heur = |cell: u32, px: i8, ax: u8, py: i8, ay: u8| -> u32 {
        let (x, y) = cells[cell as usize];
        per_axis(x, goal.0, px, ax).max(per_axis(y, goal.1, py, ay))
    };
    let start_cell = cell_id[&(0, 0)];
    // both lists must begin with a pace-0 entry: start inside that entry
    let start = encode(start_cell, 0, 1, 0, 1);
    best[start] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((heur(start_cell, 0, 1, 0, 1), 0u32, start as u32)));
    let goal_cell = cell_id.get(&goal).copied()?;
    while let Some(Reverse((f, g, idx))) = heap.pop() {
        if f as usize > limit {
            return None;
        }
        let idx = idx as usize;
        if best[idx] < g {
            continue;
        }
        let cell = (idx / (PHASES * PHASES)) as u32;
        let (px, ax) = phase_decode((idx / PHASES) % PHASES);
        let (py, ay) = phase_decode(idx % PHASES);
        if cell == goal_cell && px == 0 && py == 0 && ax == 0 && ay == 0 {
            let (mut xp, mut yp) = (Vec::new(), Vec::new());
            let mut cur = idx;
            while parent[cur] != u32::MAX {
                if entry_x[cur] != i8::MIN { xp.push(entry_x[cur] as i32) }
                if entry_y[cur] != i8::MIN { yp.push(entry_y[cur] as i32) }
                cur = parent[cur] as usize;
            }
            xp.push(0);
            yp.push(0);
            xp.reverse();
            yp.reverse();
            return Some(Found { xp, yp, time: g as usize });
        }
        let (x, y) = cells[cell as usize];
        let xs: Vec<(i8, u8, i8)> = if ax == 0 {
            next_paces(px).into_iter().map(|p| (p, p.unsigned_abs().max(1), p)).collect()
        } else {
            vec![(px, ax, i8::MIN)]
        };
        let ys: Vec<(i8, u8, i8)> = if ay == 0 {
            next_paces(py).into_iter().map(|p| (p, p.unsigned_abs().max(1), p)).collect()
        } else {
            vec![(py, ay, i8::MIN)]
        };
        for &(npx, nax0, ex) in &xs {
            for &(npy, nay0, ey) in &ys {
                let (nax, nay) = (nax0 - 1, nay0 - 1);
                let nx = x + if nax == 0 { (npx as i32).signum() } else { 0 };
                let ny = y + if nay == 0 { (npy as i32).signum() } else { 0 };
                let Some(&ncell) = cell_id.get(&(nx, ny)) else { continue };
                let nidx = encode(ncell, npx, nax, npy, nay);
                let ng = g + 1;
                if ng < best[nidx] {
                    best[nidx] = ng;
                    parent[nidx] = idx as u32;
                    entry_x[nidx] = ex;
                    entry_y[nidx] = ey;
                    heap.push(Reverse((ng + heur(ncell, npx, nax, npy, nay), ng, nidx as u32)));
                }
            }
        }
    }
    None
}
