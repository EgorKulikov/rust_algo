//! Level 6, second planner: two emitter pairs work concurrently.
//!
//! Each pair steers towards its own target cell with a small per-tick rule
//! (slide when both are on and close, otherwise keep one emitter on, move,
//! and toggle the other on when in place). The two pair lines are merged and
//! the tick is validated by simulating the beams: if any cell outside the
//! pattern would get matter, or any matter would be destroyed, the second
//! pair is paused for that tick with its emitters switched off. Pattern cells
//! created by cross-pair crossings count as free progress.

use crate::board::direction;
use crate::level4::Machine;
use crate::level5::Pattern;
use std::collections::{HashMap, HashSet};

pub struct Shared {
    pub m: Machine,
    pub ring_index: HashMap<(i32, i32), usize>,
    pub pattern: HashSet<(i32, i32)>,
    /// Phase flag: allow creating non-pattern matter (to be erased later) and
    /// blocked pattern cells.
    pub allow_stray: bool,
}

impl Shared {
    pub fn new(radius: usize, pattern: &Pattern) -> Shared {
        let m = Machine::new(radius);
        let ring_index = m.ring.iter().enumerate().map(|(i, &c)| (c, i)).collect();
        Shared { m, ring_index, pattern: pattern.cells.iter().copied().collect(), allow_stray: false }
    }

    pub(crate) fn stand(&self, e: usize, cell: (i32, i32)) -> usize {
        let (dr, dc) = direction(e as u8 + 1);
        let (mut r, mut c) = cell;
        while matches!(self.m.board.get(r, c), b'O' | b'M') {
            r -= dr;
            c -= dc;
        }
        self.ring_index[&(r, c)]
    }

    pub(crate) fn delta(&self, from: usize, to: usize) -> i32 {
        let n = self.m.ring.len() as i32;
        let d = ((to as i32 - from as i32) % n + n) % n;
        if d <= n - d { d } else { d - n }
    }

    /// Rough ticks for pair (a, b) to create `cell` from the current state.
    pub(crate) fn estimate(&self, cell: (i32, i32), a: usize, b: usize) -> usize {
        let da = self.delta(self.m.pos[a], self.stand(a, cell)).unsigned_abs() as usize;
        let db = self.delta(self.m.pos[b], self.stand(b, cell)).unsigned_abs() as usize;
        let (on_a, on_b) = (self.m.active[a], self.m.active[b]);
        if on_a && on_b && da <= 1 && db <= 1 {
            return 1;
        }
        let mut best = usize::MAX;
        for (k, dk, do_, on_o) in [(a, da, db, on_b), (b, db, da, on_a)] {
            if self.m.active[k] {
                let t_o = if on_o { if do_ == 0 { 2 } else { do_ + 2 } } else { do_ + 1 };
                best = best.min(dk.max(t_o).max(1));
            }
        }
        let need = |on: bool, d: usize| if on { if d == 0 { 2 } else { d + 2 } } else { d + 1 };
        best.min(need(on_a, da).max(need(on_b, db)))
    }

    /// Would this line create matter outside the pattern or destroy any?
    pub(crate) fn valid(&self, line: &[u8; 6]) -> bool {
        let n = self.m.ring.len();
        let mut pos = self.m.pos;
        let mut active = self.m.active;
        for k in 0..6 {
            match line[k] {
                b'F' => pos[k] = (pos[k] + 1) % n,
                b'B' => pos[k] = (pos[k] + n - 1) % n,
                b'T' => active[k] = !active[k],
                _ => {}
            }
        }
        self.valid_state(&pos, &active)
    }

    /// Number of pattern cells a hypothetical state would create, or None if
    /// firing it is not harmless.
    pub(crate) fn creations(&self, pos: &[usize; 6], active: &[bool; 6]) -> Option<usize> {
        if !self.valid_state(pos, active) {
            return None;
        }
        let mut counts: HashMap<(i32, i32), u32> = HashMap::new();
        for k in 0..6 {
            if !active[k] {
                continue;
            }
            let (dr, dc) = direction(k as u8 + 1);
            let (mut r, mut c) = self.m.ring[pos[k]];
            r += dr;
            c += dc;
            while matches!(self.m.board.get(r, c), b'O' | b'M') {
                *counts.entry((r, c)).or_insert(0) += 1;
                r += dr;
                c += dc;
            }
        }
        Some(counts.iter().filter(|(&cell, &k)| k == 2 && self.m.board.get(cell.0, cell.1) == b'O' && self.pattern.contains(&cell)).count())
    }

    /// Beam counts of a hypothetical emitter state, and whether firing it is harmless.
    pub(crate) fn valid_state(&self, pos: &[usize; 6], active: &[bool; 6]) -> bool {
        let mut counts: HashMap<(i32, i32), u32> = HashMap::new();
        for k in 0..6 {
            if !active[k] {
                continue;
            }
            let (dr, dc) = direction(k as u8 + 1);
            let (mut r, mut c) = self.m.ring[pos[k]];
            r += dr;
            c += dc;
            while matches!(self.m.board.get(r, c), b'O' | b'M') {
                *counts.entry((r, c)).or_insert(0) += 1;
                r += dr;
                c += dc;
            }
        }
        counts.iter().all(|(&cell, &k)| {
            let m = self.m.board.get(cell.0, cell.1) == b'M';
            let pat = self.pattern.contains(&cell);
            if k >= 3 {
                // never destroy pattern matter; blocking a pattern cell's creation only while filling
                !(m && pat) && (!pat || m || self.allow_stray)
            } else if k == 2 {
                m || pat || self.allow_stray
            } else {
                true
            }
        })
    }
}

/// One-tick creation of a whole hex line by an opposing emitter pair, with
/// extra (non-pattern) cells on the line blocked by a third beam each.
pub struct Fill {
    /// (emitter, ring position it must stand at)
    pub stands: Vec<(usize, usize)>,
    pub new_cells: usize,
}

const AXES: [(usize, usize); 3] = [(0, 3), (1, 4), (2, 5)];

impl Shared {
    /// Cells of the hex line through `cell` along the axis of emitter `e`.
    pub(crate) fn line_cells(&self, cell: (i32, i32), e: usize) -> Vec<(i32, i32)> {
        let (dr, dc) = direction(e as u8 + 1);
        let (mut r, mut c) = cell;
        while matches!(self.m.board.get(r - dr, c - dc), b'O' | b'M') {
            r -= dr;
            c -= dc;
        }
        let mut cells = Vec::new();
        while matches!(self.m.board.get(r, c), b'O' | b'M') {
            cells.push((r, c));
            r += dr;
            c += dc;
        }
        cells
    }

    /// Ticks to execute a fill from the current state: everyone involved
    /// travels switched off, then all toggle on together.
    pub(crate) fn fill_ticks(&self, f: &Fill) -> usize {
        let mut t = 1;
        for &(e, stand) in &f.stands {
            let d = self.delta(self.m.pos[e], stand).unsigned_abs() as usize;
            let need = if self.m.active[e] { if d == 0 { 2 } else { d + 2 } } else { d + 1 };
            t = t.max(need);
        }
        t
    }

    /// Best fill through the remaining cells, by cells created per tick.
    fn best_fill(&self, remaining: &[(i32, i32)]) -> Option<(Fill, usize)> {
        let mut seen: HashSet<(usize, (i32, i32))> = HashSet::new();
        let mut best: Option<(Fill, usize, f64)> = None;
        for &cell in remaining {
            for (axis, &(e1, e2)) in AXES.iter().enumerate() {
                let cells = self.line_cells(cell, e1);
                if !seen.insert((axis, cells[0])) {
                    continue;
                }
                let new_cells = cells.iter().filter(|&&(r, c)| self.pattern.contains(&(r, c)) && self.m.board.get(r, c) == b'O').count();
                let extras: Vec<(i32, i32)> = cells.iter().copied().filter(|c| !self.pattern.contains(c)).collect();
                if new_cells < 2 || extras.len() > 2 {
                    continue;
                }
                let blockers: Vec<usize> = (0..6).filter(|&e| e != e1 && e != e2).collect();
                // try every assignment of distinct blocker emitters to the extras
                let mut assignments: Vec<Vec<usize>> = vec![vec![]];
                for _ in 0..extras.len() {
                    let mut next = Vec::new();
                    for a in &assignments {
                        for &b in &blockers {
                            if !a.contains(&b) {
                                let mut v = a.clone();
                                v.push(b);
                                next.push(v);
                            }
                        }
                    }
                    assignments = next;
                }
                for assign in assignments {
                    let mut stands = vec![(e1, self.stand(e1, cells[0])), (e2, self.stand(e2, *cells.last().unwrap()))];
                    for (i, &b) in assign.iter().enumerate() {
                        stands.push((b, self.stand(b, extras[i])));
                    }
                    // the fire tick must be harmless with exactly these emitters on
                    let mut pos = self.m.pos;
                    let mut active = [false; 6];
                    for &(e, st) in &stands {
                        pos[e] = st;
                        active[e] = true;
                    }
                    if !self.valid_state(&pos, &active) {
                        continue;
                    }
                    let f = Fill { stands, new_cells };
                    let t = self.fill_ticks(&f);
                    let rate = new_cells as f64 / t as f64;
                    if best.as_ref().map_or(true, |b| rate > b.2) {
                        best = Some((f, t, rate));
                    }
                }
            }
        }
        best.map(|(f, t, _)| (f, t))
    }

    /// Like `fill_line`, but emitters already in place keep their state and
    /// only movers are switched off; the fire tick toggles on whoever is off.
    /// Must be validated by the caller.
    pub(crate) fn fill_line_warm(&self, f: &Fill) -> ([u8; 6], bool) {
        let mut line = [b'-'; 6];
        let involved: Vec<usize> = f.stands.iter().map(|&(e, _)| e).collect();
        for e in 0..6 {
            if !involved.contains(&e) && self.m.active[e] {
                line[e] = b'T';
            }
        }
        let mut all_in_place = true;
        for &(e, st) in &f.stands {
            let d = self.delta(self.m.pos[e], st);
            if d != 0 {
                all_in_place = false;
                if self.m.active[e] {
                    line[e] = b'T';
                } else {
                    line[e] = if d > 0 { b'F' } else { b'B' };
                }
            }
        }
        if all_in_place {
            for &(e, _) in &f.stands {
                if !self.m.active[e] {
                    line[e] = b'T';
                }
            }
            // if everyone is already on, this tick simply fires again
        }
        (line, all_in_place)
    }

    /// Next line of a fill in progress; `true` when this line is the fire tick.
    pub(crate) fn fill_line(&self, f: &Fill) -> ([u8; 6], bool) {
        let mut line = [b'-'; 6];
        let involved: Vec<usize> = f.stands.iter().map(|&(e, _)| e).collect();
        for e in 0..6 {
            if !involved.contains(&e) && self.m.active[e] {
                line[e] = b'T';
            }
        }
        let mut all_ready = true;
        for &(e, st) in &f.stands {
            let d = self.delta(self.m.pos[e], st);
            if self.m.active[e] {
                line[e] = b'T';
                all_ready = false;
            } else if d != 0 {
                line[e] = if d > 0 { b'F' } else { b'B' };
                all_ready = false;
            }
        }
        if all_ready {
            for &(e, _) in &f.stands {
                line[e] = b'T';
            }
        }
        (line, all_ready)
    }
}

pub struct PairPlan {
    pub a: usize,
    pub b: usize,
    pub target: Option<(i32, i32)>,
    pub stalls: usize,
}

impl PairPlan {
    /// Candidate instruction lines for a and b (others '-'), best first. The
    /// last candidate never creates stray matter on its own; earlier ones keep
    /// both beams on while moving and need the tick to be validated.
    pub(crate) fn next_line(&self, s: &Shared, cell: (i32, i32)) -> Vec<[u8; 6]> {
        let (a, b) = (self.a, self.b);
        let mut line = [b'-'; 6];
        let da = s.delta(s.m.pos[a], s.stand(a, cell));
        let db = s.delta(s.m.pos[b], s.stand(b, cell));
        let step = |d: i32| if d > 0 { b'F' } else if d < 0 { b'B' } else { b'-' };
        let (on_a, on_b) = (s.m.active[a], s.m.active[b]);
        if on_a && on_b {
            if da.abs() <= 1 && db.abs() <= 1 {
                line[a] = step(da);
                line[b] = step(db);
            } else {
                // preferred: keep both on and step both (intermediate crossing must be harmless)
                let mut slide = [b'-'; 6];
                slide[a] = step(da.signum());
                slide[b] = step(db.signum());
                // fallback: switch off the one with the longer way, move the other one step
                let (off, mv, dm) = if da.abs() >= db.abs() { (a, b, db) } else { (b, a, da) };
                line[off] = b'T';
                line[mv] = step(dm.signum());
                return vec![slide, line];
            }
        } else if on_a || on_b {
            let (k, o, dk, do_) = if on_a { (a, b, da, db) } else { (b, a, db, da) };
            if do_ == 0 && dk.abs() <= 1 {
                line[o] = b'T';
                line[k] = step(dk);
            } else if do_ == 0 {
                // preferred: switch o on now and slide k the rest of the way
                let mut slide = [b'-'; 6];
                slide[o] = b'T';
                slide[k] = step(dk.signum());
                line[k] = step(dk.signum());
                return vec![slide, line];
            } else {
                line[o] = step(do_.signum());
                line[k] = step(dk.signum());
            }
        } else if da == 0 && db == 0 {
            line[a] = b'T';
            line[b] = b'T';
        } else if da == 0 {
            line[a] = b'T';
            line[b] = step(db.signum());
        } else if db == 0 {
            line[b] = b'T';
            line[a] = step(da.signum());
        } else {
            line[a] = step(da.signum());
            line[b] = step(db.signum());
        }
        vec![line]
    }

    /// Secondary pair: travel with both emitters off, fire only when both
    /// stand at the target (both toggled on in the same tick).
    pub(crate) fn next_line_quiet(&self, s: &Shared, cell: (i32, i32)) -> [u8; 6] {
        let (a, b) = (self.a, self.b);
        let mut line = [b'-'; 6];
        let da = s.delta(s.m.pos[a], s.stand(a, cell));
        let db = s.delta(s.m.pos[b], s.stand(b, cell));
        let step = |d: i32| if d > 0 { b'F' } else if d < 0 { b'B' } else { b'-' };
        if s.m.active[a] || s.m.active[b] {
            return self.off_line(s);
        }
        if da == 0 && db == 0 {
            line[a] = b'T';
            line[b] = b'T';
        } else {
            line[a] = step(da.signum());
            line[b] = step(db.signum());
        }
        line
    }

    pub(crate) fn off_line(&self, s: &Shared) -> [u8; 6] {
        let mut line = [b'-'; 6];
        for e in [self.a, self.b] {
            if s.m.active[e] {
                line[e] = b'T';
            }
        }
        line
    }
}

pub(crate) fn merge(x: &[u8; 6], y: &[u8; 6]) -> [u8; 6] {
    let mut r = *x;
    for k in 0..6 {
        if y[k] != b'-' {
            r[k] = y[k];
        }
    }
    r
}

/// Builds `count` copies with two pairs; returns the program.
pub fn build(radius: usize, pattern: &Pattern, count: usize, pairs: [(usize, usize); 2], rng: &mut u64, noise: u64) -> Vec<[u8; 6]> {
    let mut s = Shared::new(radius, pattern);
    let mut plans = [
        PairPlan { a: pairs[0].0, b: pairs[0].1, target: None, stalls: 0 },
        PairPlan { a: pairs[1].0, b: pairs[1].1, target: None, stalls: 0 },
    ];
    let mut lines = Vec::new();
    let mut fill: Option<Fill> = None;
    for _ in 0..count {
        loop {
            let remaining: Vec<(i32, i32)> = pattern.cells.iter().copied().filter(|&(r, c)| s.m.board.get(r, c) == b'O').collect();
            if remaining.is_empty() {
                break;
            }
            // (re)assign targets: drop targets that already exist or were claimed by the other pair
            for i in 0..2 {
                if let Some(t) = plans[i].target {
                    if s.m.board.get(t.0, t.1) == b'M' {
                        plans[i].target = None;
                    }
                }
            }
            for i in 0..2 {
                if plans[i].target.is_none() {
                    let other = plans[1 - i].target;
                    let (a, b) = (plans[i].a, plans[i].b);
                    let mut best: Option<((i32, i32), usize)> = None;
                    for &cell in &remaining {
                        if Some(cell) == other && remaining.len() > 1 {
                            continue;
                        }
                        *rng ^= *rng << 13; *rng ^= *rng >> 7; *rng ^= *rng << 17;
                        let e = s.estimate(cell, a, b) * 1000 + (*rng % (1 + noise)) as usize;
                        if best.map_or(true, |(_, be)| e < be) {
                            best = Some((cell, e));
                        }
                    }
                    plans[i].target = best.map(|(c, _)| c);
                }
            }
            if fill.is_none() && std::env::var("NANO_NOFILL").is_err() {
                if let Some((f, t)) = s.best_fill(&remaining) {
                    let (a, b) = (plans[0].a, plans[0].b);
                    let single = remaining.iter().map(|&c| s.estimate(c, a, b)).min().unwrap_or(1).max(1);
                    if (t as f64) < f.new_cells as f64 * single as f64 * 0.95 {
                        fill = Some(f);
                    }
                }
            }
            if let Some(f) = &fill {
                let (line, fired) = s.fill_line(f);
                assert!(s.valid(&line), "fill line invalid");
                s.m.tick(&line);
                lines.push(line);
                if fired {
                    fill = None;
                    plans[0].target = None;
                    plans[1].target = None;
                }
                continue;
            }
            let mut cands = plans[0].next_line(&s, plans[0].target.unwrap());
            // emitters belonging to neither pair (fill blockers) must go dark again
            for cand in cands.iter_mut() {
                for e in 0..6 {
                    let in_pairs = [plans[0].a, plans[0].b, plans[1].a, plans[1].b].contains(&e);
                    if !in_pairs && s.m.active[e] {
                        cand[e] = b'T';
                    }
                }
            }
            let mut c2 = plans[1].next_line(&s, plans[1].target.unwrap());
            c2.push(plans[1].next_line_quiet(&s, plans[1].target.unwrap()));
            let off2 = plans[1].off_line(&s);
            let mut chosen = None;
            'outer: for l1 in &cands {
                for l2 in &c2 {
                    let line = merge(l1, l2);
                    if s.valid(&line) {
                        chosen = Some((line, false));
                        break 'outer;
                    }
                }
                let line = merge(l1, &off2);
                if s.valid(&line) {
                    chosen = Some((line, true));
                    break;
                }
            }
            let (line, stalled) = chosen.expect("no valid line for the primary pair");
            if stalled {
                plans[1].stalls += 1;
                if plans[1].stalls > 12 {
                    plans[1].target = None;
                    plans[1].stalls = 0;
                }
            } else {
                plans[1].stalls = 0;
            }
            if std::env::var("NANO_DEBUG6").is_ok() && (lines.len() < 80 || lines.len() > 199_960) {
                eprintln!("{:>3} {} targets={:?} pos={:?} act={:?}", lines.len(), std::str::from_utf8(&line).unwrap(), [plans[0].target, plans[1].target], s.m.pos, s.m.active);
            }
            s.m.tick(&line);
            lines.push(line);
            assert!(lines.len() < 200_000, "runaway program");
        }
        assert!(Pattern::new(pattern.cells.clone()).is_valid(&s.m));
        pattern.remove(&mut s.m);
    }
    lines
}
