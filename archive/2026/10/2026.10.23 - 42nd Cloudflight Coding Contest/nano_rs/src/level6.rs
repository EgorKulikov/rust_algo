//! Level 6: generate short programs that build each pattern `count` times.
//!
//! With exactly two emitters on, a tick creates matter only at their beams'
//! crossing, so every pattern cell is produced by one "creation tick" with a
//! non-parallel emitter pair aimed at it. Between creations at most one
//! emitter stays on (a single beam never creates matter), the others move in
//! parallel, and toggles are merged into movement ticks. Cells are ordered
//! greedily by the number of ticks they cost from the current emitter state.
//! Every program is re-simulated with the level 4/5 machine before output.

use crate::board::direction;
use crate::level4::Machine;
use crate::level5::Pattern;
use std::collections::HashMap;

pub struct Job {
    pub radius: usize,
    pub pattern: Pattern,
    pub count: usize,
}

fn parse_jobs(input: &str) -> Vec<Job> {
    let mut lines = input.lines().map(str::trim_end).filter(|l| !l.trim().is_empty());
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut jobs = Vec::with_capacity(n);
    for _ in 0..n {
        let dims: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut cells = Vec::new();
        for r in 0..dims[1] {
            for (c, &b) in lines.next().unwrap().as_bytes().iter().enumerate() {
                if b == b'M' {
                    cells.push((r as i32, c as i32));
                }
            }
        }
        jobs.push(Job { radius: dims[0], pattern: Pattern::new(cells), count: dims[3] });
    }
    jobs
}

/// Emitter pairs that are not parallel (1|4, 2|5, 3|6 are parallel).
const PAIRS: [(usize, usize); 12] = [(0, 1), (0, 2), (0, 4), (0, 5), (1, 2), (1, 3), (1, 5), (2, 3), (2, 4), (3, 4), (3, 5), (4, 5)];

struct Planner {
    m: Machine,
    ring_index: HashMap<(i32, i32), usize>,
    lines: Vec<[u8; 6]>,
}

impl Planner {
    fn new(radius: usize) -> Planner {
        let m = Machine::new(radius);
        let ring_index = m.ring.iter().enumerate().map(|(i, &c)| (c, i)).collect();
        Planner { m, ring_index, lines: Vec::new() }
    }

    /// Ring index where emitter `e` must stand to fire through `cell`.
    fn stand(&self, e: usize, cell: (i32, i32)) -> usize {
        let (dr, dc) = direction(e as u8 + 1);
        let (mut r, mut c) = cell;
        while matches!(self.m.board.get(r, c), b'O' | b'M') {
            r -= dr;
            c -= dc;
        }
        self.ring_index[&(r, c)]
    }

    /// Signed shortest ring move from `from` to `to`: positive = clockwise.
    fn delta(&self, from: usize, to: usize) -> i32 {
        let n = self.m.ring.len() as i32;
        let d = ((to as i32 - from as i32) % n + n) % n;
        if d <= n - d { d } else { d - n }
    }

    /// Ticks needed to create `cell` with emitters (a, b) from the current
    /// state, and which of them (if any) stays on meanwhile.
    fn cost(&self, cell: (i32, i32), a: usize, b: usize) -> (usize, Option<usize>) {
        let da = self.delta(self.m.pos[a], self.stand(a, cell)).unsigned_abs() as usize;
        let db = self.delta(self.m.pos[b], self.stand(b, cell)).unsigned_abs() as usize;
        let (on_a, on_b) = (self.m.active[a], self.m.active[b]);
        let mut best = (usize::MAX, None);
        // both on and at most one ring step away each: a single tick moves
        // them and the beams cross at the new cell ("slide"); no toggles
        if on_a && on_b && da <= 1 && db <= 1 {
            return (1, None);
        }
        let mut consider = |t: usize, keep: Option<usize>| {
            if t < best.0 {
                best = (t, keep);
            }
        };
        // keep `k` on (must be on now); the other `o` toggles on in the last tick
        for (k, o, dk, do_) in [(a, b, da, db), (b, a, db, da)] {
            if !self.m.active[k] {
                continue;
            }
            // k moves in ticks 1..=T, o: if on now it goes off in tick 1 and moves in 2..T-1,
            // else it moves in 1..T-1; toggles on at T.
            let t_o = if self.m.active[o] { if do_ == 0 { 2 } else { do_ + 2 } } else { do_ + 1 };
            consider(dk.max(t_o).max(1), Some(k));
        }
        // nobody kept on: both toggle on in the last tick
        let need = |e: usize, d: usize| if self.m.active[e] { if d == 0 { 2 } else { d + 2 } } else { d + 1 };
        consider(need(a, da).max(need(b, db)), None);
        best
    }

    /// Emits the ticks that create `cell` with (a, b); `keep` stays on.
    fn create(&mut self, cell: (i32, i32), a: usize, b: usize, keep: Option<usize>, t: usize) {
        let mut plan = vec![[b'-'; 6]; t];
        if t == 1 && keep.is_none() && self.m.active[a] && self.m.active[b] {
            // slide: both stay on, each moves at most one step; switch off
            // any other emitter
            for e in 0..6 {
                if e == a || e == b {
                    let d = self.delta(self.m.pos[e], self.stand(e, cell));
                    plan[0][e] = match d { 0 => b'-', 1 => b'F', -1 => b'B', _ => unreachable!() };
                } else if self.m.active[e] {
                    plan[0][e] = b'T';
                }
            }
            self.m.tick(&plan[0]);
            self.lines.push(plan[0]);
            return;
        }
        let mut moves = [0i32; 6];
        for e in 0..6 {
            if e == a || e == b {
                moves[e] = self.delta(self.m.pos[e], self.stand(e, cell));
            }
        }
        for e in 0..6 {
            let in_pair = e == a || e == b;
            if !in_pair {
                if self.m.active[e] {
                    plan[0][e] = b'T';
                }
                continue;
            }
            // tick range where e may move, and its toggle ticks
            let (first, last) = if Some(e) == keep {
                (0, t)
            } else if self.m.active[e] {
                if t >= 2 { plan[0][e] = b'T'; }
                plan[t - 1][e] = b'T';
                (1, t - 1)
            } else {
                plan[t - 1][e] = b'T';
                (0, t - 1)
            };
            // special case: on now, in place, no keeper: "off" then "on" needs t >= 2
            let mut remaining = moves[e].unsigned_abs() as usize;
            let step = if moves[e] > 0 { b'F' } else { b'B' };
            for tick in first..last {
                if remaining == 0 {
                    break;
                }
                plan[tick][e] = step;
                remaining -= 1;
            }
            assert_eq!(remaining, 0, "cannot fit moves of emitter {} in {} ticks", e, t);
        }
        for line in plan {
            self.m.tick(&line);
            self.lines.push(line);
        }
    }

    fn build(&mut self, job: &Job, rng: &mut u64, noise: u32) {
        for _ in 0..job.count {
            let mut remaining: Vec<(i32, i32)> = job.pattern.cells.clone();
            while !remaining.is_empty() {
                // (cost with random tie-breaking noise, real cost, index, pair, keep)
                let mut best = (u64::MAX, usize::MAX, 0usize, (0, 0), None);
                for (i, &cell) in remaining.iter().enumerate() {
                    for &(a, b) in &PAIRS {
                        let (t, keep) = self.cost(cell, a, b);
                        *rng ^= *rng << 13; *rng ^= *rng >> 7; *rng ^= *rng << 17;
                        let key = (t as u64) * 1000 + (*rng % (1 + noise as u64));
                        if key < best.0 {
                            best = (key, t, i, (a, b), keep);
                        }
                    }
                }
                let (_, t, i, (a, b), keep) = best;
                let cell = remaining.swap_remove(i);
                let before = self.m.board.get(cell.0, cell.1);
                self.create(cell, a, b, keep, t);
                debug_assert_eq!(before, b'O');
                assert_eq!(self.m.board.get(cell.0, cell.1), b'M', "cell {:?} not created", cell);
            }
            assert!(job.pattern.is_valid(&self.m), "pattern not valid after building");
            job.pattern.remove(&mut self.m);
        }
    }
}

/// Independent check: replay the program on a fresh machine with the level 5
/// rules and count submissions.
fn verify(job: &Job, lines: &[[u8; 6]]) -> usize {
    let mut m = Machine::new(job.radius);
    let mut count = 0;
    for line in lines {
        m.tick(line);
        if job.pattern.is_valid(&m) {
            job.pattern.remove(&mut m);
            count += 1;
        }
    }
    count
}

pub fn solve(input: &str) -> String {
    let jobs = parse_jobs(input);
    let mut out = String::new();
    let mut total = 0;
    let threads: usize = std::env::var("NANO_THREADS").ok().and_then(|v| v.parse().ok()).unwrap_or(16);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let partial_lock = std::sync::Mutex::new(());
    let results: Vec<std::sync::Mutex<Vec<[u8; 6]>>> = jobs.iter().map(|_| std::sync::Mutex::new(Vec::new())).collect();
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if i >= jobs.len() {
                    break;
                }
                let lines = solve_job(i, &jobs[i]);
                if let Ok(path) = std::env::var("NANO_PARTIAL") {
                    if verify(&jobs[i], &lines) == jobs[i].count {
                        use std::io::Write;
                        let mut text = format!("#job {} {}\n", i, lines.len());
                        for line in &lines {
                            let s: Vec<String> = line.iter().map(|&b| (b as char).to_string()).collect();
                            text.push_str(&s.join(" "));
                            text.push('\n');
                        }
                        text.push('\n');
                        let _g = partial_lock.lock().unwrap();
                        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
                            let _ = f.write_all(text.as_bytes());
                        }
                    }
                }
                *results[i].lock().unwrap() = lines;
            });
        }
    });
    for (i, job) in jobs.iter().enumerate() {
        let best_lines = results[i].lock().unwrap().clone();
        let got = verify(job, &best_lines);
        assert_eq!(got, job.count, "job {}: program yields {} patterns, need {}", i, got, job.count);
        total += best_lines.len();
        for line in &best_lines {
            let s: Vec<String> = line.iter().map(|&b| (b as char).to_string()).collect();
            out.push_str(&s.join(" "));
            out.push('\n');
        }
        out.push('\n');
    }
    eprintln!("total lines: {}", total);
    out
}

fn solve_job(i: usize, job: &Job) -> Vec<[u8; 6]> {
    let restarts: usize = std::env::var("NANO_RESTARTS").ok().and_then(|v| v.parse().ok()).unwrap_or(20);
    {
        let mut rng = 0x9E37_79B9_7F4A_7C15u64 ^ (i as u64 + 1);
        let mut p = Planner::new(job.radius);
        p.build(job, &mut rng, 0);
        for k in 0..restarts {
            let mut q = Planner::new(job.radius);
            q.build(job, &mut rng, 1 + (k as u32 % 3) * 500);
            if q.lines.len() < p.lines.len() {
                p = q;
            }
        }
        let mut best_lines = p.lines.clone();
        let twin_restarts: usize = std::env::var("NANO_TWIN").ok().and_then(|v| v.parse().ok()).unwrap_or(4);
        for pairs in [[(0, 1), (3, 4)], [(0, 2), (3, 5)], [(1, 2), (4, 5)], [(3, 4), (0, 1)], [(3, 5), (0, 2)], [(4, 5), (1, 2)]] {
            for k in 0..=twin_restarts {
                let noise = if k == 0 { 0 } else { [1, 300, 800][k % 3] };
                let lines = crate::level6b::build(job.radius, &job.pattern, job.count, pairs, &mut rng, noise);
                if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                    best_lines = lines;
                }
            }
        }
        let pair_sets: Vec<Vec<(usize, usize)>> = vec![
            vec![(0, 1), (3, 4)], vec![(0, 2), (3, 5)], vec![(1, 2), (4, 5)],
            vec![(0, 1), (4, 2), (5, 3)], vec![(0, 2), (1, 5), (4, 3)], vec![(3, 1), (0, 5), (4, 2)],
        ];
        for pairs in &pair_sets {
            for (erase_cost, single_cost) in [(1.5, 0.9), (2.5, 0.9), (4.0, 0.9), (6.0, 1.0), (9.0, 1.0), (12.0, 1.1)] {
                let lines = crate::level6c::build(job.radius, &job.pattern, job.count, pairs, erase_cost, single_cost);
                if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                    best_lines = lines;
                }
            }
        }
        for axis in 0..3 {
            for pairs in [&pair_sets[0], &pair_sets[3]] {
                let lines = crate::level6c::build_mode(job.radius, &job.pattern, job.count, pairs, 2.0, 1.0, None, Some(axis));
                if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                    best_lines = lines;
                }
            }
        }
        for rotation in 0..8usize {
            for max_idle in [8usize, 20] {
                let triple = move |s: &mut crate::level6b::Shared, p: &crate::level5::Pattern, l: &mut Vec<[u8; 6]>| crate::level6g::triple_phase(s, p, l, rotation, max_idle);
                for sc in [1.0, 0.0] {
                    let lines = crate::level6c::build_with(job.radius, &job.pattern, job.count, &pair_sets[3], 9.0, sc, Some(&triple));
                    if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                        best_lines = lines;
                    }
                }
            }
        }
        for hub_axis in 0..3 {
            for sweep_axis in 0..3 {
                if sweep_axis == hub_axis {
                    continue;
                }
                for min_missing in [2usize, 3] {
                    let dual = move |s: &mut crate::level6b::Shared, p: &crate::level5::Pattern, l: &mut Vec<[u8; 6]>| crate::level6e::dual_phase(s, p, l, hub_axis, sweep_axis, min_missing);
                    for &(ec, sc) in &[(9.0, 1.0), (9.0, 0.5)] {
                        let lines = crate::level6c::build_with(job.radius, &job.pattern, job.count, &pair_sets[3], ec, sc, Some(&dual));
                        if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                            best_lines = lines;
                        }
                    }
                }
            }
        }
        for axis in 0..3 {
            for min_missing in [2usize, 4] {
                for pairs in [&pair_sets[0], &pair_sets[3]] {
                    let hub = move |s: &mut crate::level6b::Shared, p: &crate::level5::Pattern, l: &mut Vec<[u8; 6]>| crate::level6d::hub_phase(s, p, l, axis, min_missing);
                    let lines = crate::level6c::build_with(job.radius, &job.pattern, job.count, pairs, 9.0, 1.0, Some(&hub));
                    if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                        best_lines = lines;
                    }
                }
            }
        }
        let beam_width: usize = std::env::var("NANO_BEAM").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
        if beam_width > 0 {
            let before = best_lines.len();
            // NANO_BEAM_SETS="stray_pen,pot_w,cell_w;..." (default one set 1500,10,1000)
            let sets: Vec<crate::level6h::Params> = std::env::var("NANO_BEAM_SETS")
                .unwrap_or_else(|_| "1500,10,1000".to_string())
                .split(';')
                .map(|t| {
                    let v: Vec<i64> = t.split(',').map(|x| x.trim().parse().unwrap()).collect();
                    let cap: usize = std::env::var("NANO_BEAM_CAP").ok().and_then(|x| x.parse().ok()).unwrap_or(0);
                    // width grows with the repetition count (its cycle is paid `count` times)
                    let scale: usize = std::env::var("NANO_BEAM_SCALE").ok().and_then(|x| x.parse().ok()).unwrap_or(0);
                    let mut width = beam_width;
                    if scale > 0 {
                        width = beam_width * (1 + job.count.min(scale) / 2);
                        if job.radius >= 9 {
                            width /= 2;
                        }
                    }
                    crate::level6h::Params { width, stray_pen: v[0], pot_w: v[1], cell_w: v[2], cap, guide: false }
                })
                .collect();
            let guide_min: usize = std::env::var("NANO_GUIDE").ok().and_then(|x| x.parse().ok()).unwrap_or(usize::MAX);
            let mut sets = sets;
            if job.pattern.cells.len() >= guide_min {
                let mut g = sets.clone();
                for prm in g.iter_mut() {
                    prm.guide = true;
                }
                if std::env::var("NANO_GUIDE_ONLY").is_ok() {
                    sets = g;
                } else {
                    sets.extend(g);
                }
            }
            for prm in sets {
                let cap: usize = std::env::var("NANO_BEAM_TICKS").ok().and_then(|x| x.parse().ok()).unwrap_or(best_lines.len().saturating_sub(1));
                if let Some(lines) = crate::level6h::beam_build(job.radius, &job.pattern, job.count, prm, cap) {
                    if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                        best_lines = lines;
                    }
                }
                // dense patterns: sweep-fill then beam endgame
                let min_cells: usize = std::env::var("NANO_HYBRID").ok().and_then(|x| x.parse().ok()).unwrap_or(usize::MAX);
                if job.pattern.cells.len() >= min_cells {
                    if let Some(lines) = crate::level6j::hybrid_build(job.radius, &job.pattern, job.count, prm, best_lines.len().saturating_sub(1)) {
                        if (lines.len() < best_lines.len() || std::env::var("NANO_HYBRID_FORCE").is_ok()) && verify(job, &lines) == job.count {
                            best_lines = lines;
                        }
                    }
                }
            }
            if std::env::var("NANO_STATS").is_ok() {
                eprintln!("job {} r={} cells={} x{}: beam {} vs {}", i, job.radius, job.pattern.cells.len(), job.count, best_lines.len(), before);
            }
        }
        let exact_ms: u64 = std::env::var("NANO_EXACT").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
        let exact_r: usize = std::env::var("NANO_EXACT_R").ok().and_then(|v| v.parse().ok()).unwrap_or(3);
        if exact_ms > 0 && job.radius <= exact_r {
            let before = best_lines.len();
            let t0 = std::time::Instant::now();
            let exact = crate::level6i::exact_build(job.radius, &job.pattern, job.count, exact_ms);
            if std::env::var("NANO_STATS").is_ok() {
                eprintln!("job {} r={} cells={} x{}: exact {:?} vs {} in {:?}", i, job.radius, job.pattern.cells.len(), job.count, exact.as_ref().map(|l| l.len()), before, t0.elapsed());
            }
            if let Some(lines) = exact {
                if lines.len() < best_lines.len() && verify(job, &lines) == job.count {
                    best_lines = lines;
                }
            }
        }
        best_lines
    }
}

/// Debug: replay a program for the first job of `input`, printing the board after each tick.
pub fn trace(input: &str, program: &str) -> String {
    let jobs = parse_jobs(input);
    let job = &jobs[0];
    let mut m = Machine::new(job.radius);
    let mut count = 0;
    let mut out = String::new();
    for (t, line) in program.lines().map(str::trim).filter(|l| !l.is_empty()).enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut instr = [b'-'; 6];
        for k in 0..6 {
            instr[k] = parts[k].as_bytes()[0];
        }
        m.tick(&instr);
        let valid = job.pattern.is_valid(&m);
        out.push_str(&format!("tick {} {} pos={:?} on={:?}{}\n", t, line, m.pos.iter().map(|&p| m.ring[p]).collect::<Vec<_>>(), m.active, if valid { " SUBMIT" } else { "" }));
        out.push_str(&m.board.render());
        if valid {
            job.pattern.remove(&mut m);
            count += 1;
        }
    }
    out.push_str(&format!("submitted {} / {}\n", count, job.count));
    out
}
