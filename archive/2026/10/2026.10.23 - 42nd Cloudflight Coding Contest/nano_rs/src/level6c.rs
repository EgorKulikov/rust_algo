//! Level 6, third planner: bulk creation first.
//!
//! Per copy: (1) fill whole hex lines with an opposing emitter pair, blocking
//! up to four extra cells per line with single beams in the same tick, as
//! long as the line pays off; extras that cannot be blocked become stray
//! matter. (2) Erase every stray cell with three concurrent beams, one per
//! axis. (3) Create the remaining pattern cells with the two-pair planner.

use crate::level4::Machine;
use crate::level5::Pattern;
use crate::level6b::{merge, Fill, PairPlan, Shared};
use std::collections::HashSet;

pub(crate) const AXES: [(usize, usize); 3] = [(0, 3), (1, 4), (2, 5)];

/// Cost estimate of a fill: travel + fire tick + erasing the unblocked extras.
/// Blockers are added nearest-first; every prefix (0..=4 blockers) is costed
/// and the cheapest kept, so far-away blockers are skipped in favour of a
/// later erase.
pub(crate) fn plan_fill(s: &Shared, cells: &[(i32, i32)], e1: usize, e2: usize, erase_cost: f64) -> Option<(Fill, f64, usize)> {
    let all: Vec<usize> = (0..6).filter(|&e| e != e1 && e != e2).collect();
    plan_fill_with(s, cells, e1, e2, erase_cost, &all)
}

pub(crate) fn plan_fill_with(s: &Shared, cells: &[(i32, i32)], e1: usize, e2: usize, erase_cost: f64, allowed: &[usize]) -> Option<(Fill, f64, usize)> {
    let new_cells = cells.iter().filter(|&&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').count();
    if new_cells < 2 {
        return None;
    }
    let extras: Vec<(i32, i32)> = cells.iter().copied().filter(|&(r, c)| !s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').collect();
    let blockers: Vec<usize> = allowed.iter().copied().filter(|&e| e != e1 && e != e2).collect();
    let base = vec![(e1, s.stand(e1, cells[0])), (e2, s.stand(e2, *cells.last().unwrap()))];
    let fire_ok = |stands: &Vec<(usize, usize)>| {
        let mut pos = s.m.pos;
        let mut active = [false; 6];
        for &(e, st) in stands {
            pos[e] = st;
            active[e] = true;
        }
        s.valid_state(&pos, &active)
    };
    if !fire_ok(&base) {
        return None;
    }
    let cost_of = |stands: &Vec<(usize, usize)>, blocked: usize| {
        let t = s.fill_ticks(&Fill { stands: stands.clone(), new_cells });
        t as f64 + (extras.len() - blocked) as f64 * erase_cost
    };
    let mut best = (base.clone(), cost_of(&base, 0));
    // greedy: repeatedly add the (extra, blocker) pair with the smallest travel
    let mut stands = base;
    let mut used: Vec<usize> = Vec::new();
    let mut done: Vec<(i32, i32)> = Vec::new();
    for _ in 0..extras.len().min(4) {
        let mut pick: Option<((i32, i32), usize, i32)> = None;
        for &x in &extras {
            if done.contains(&x) {
                continue;
            }
            for &b in &blockers {
                if used.contains(&b) {
                    continue;
                }
                let d = s.delta(s.m.pos[b], s.stand(b, x)).abs();
                if pick.map_or(true, |(_, _, pd)| d < pd) {
                    pick = Some((x, b, d));
                }
            }
        }
        let Some((x, b, _)) = pick else { break };
        let mut trial = stands.clone();
        trial.push((b, s.stand(b, x)));
        done.push(x);
        if !fire_ok(&trial) {
            continue;
        }
        used.push(b);
        stands = trial;
        let c = cost_of(&stands, used.len());
        if c < best.1 {
            best = (stands.clone(), c);
        }
    }
    let (stands, cost) = best;
    Some((Fill { stands, new_cells }, cost, new_cells))
}

/// If the opposing pair (e1, e2) is on and aimed at a line, try to step both
/// ends to an adjacent line and fire it in the same tick, re-aiming blockers
/// by at most one ring step. Returns the tick line and the new cells it makes.
fn sweep_step(s: &Shared, e1: usize, e2: usize, erase_cost: f64, single_cost: f64) -> Option<([u8; 6], usize)> {
    if !(s.m.active[e1] && s.m.active[e2]) {
        return None;
    }
    let n = s.m.ring.len();
    let blockers: Vec<usize> = (0..6).filter(|&e| e != e1 && e != e2).collect();
    let mut best: Option<([u8; 6], usize)> = None;
    for dir in [1i32, -1] {
        let p1 = (s.m.pos[e1] as i32 + dir).rem_euclid(n as i32) as usize;
        // the line entered from p1 in direction e1
        let (dr, dc) = crate::board::direction(e1 as u8 + 1);
        let (r0, c0) = s.m.ring[p1];
        let first = (r0 + dr, c0 + dc);
        if !matches!(s.m.board.get(first.0, first.1), b'O' | b'M') {
            continue;
        }
        let cells = s.line_cells(first, e1);
        let p2 = s.stand(e2, *cells.last().unwrap());
        let d2 = s.delta(s.m.pos[e2], p2);
        if d2.abs() != 1 {
            continue;
        }
        let new_cells = cells.iter().filter(|&&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').count();
        if new_cells == 0 {
            continue;
        }
        let extras: Vec<(i32, i32)> = cells.iter().copied().filter(|&(r, c)| !s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').collect();
        if extras.len() > 4 {
            continue;
        }
        let mut line = [b'-'; 6];
        line[e1] = if dir > 0 { b'F' } else { b'B' };
        line[e2] = if d2 > 0 { b'F' } else { b'B' };
        let mut used: Vec<usize> = Vec::new();
        let mut strays = 0usize;
        for &x in &extras {
            let mut pick: Option<(usize, i32)> = None;
            for &b in &blockers {
                if used.contains(&b) {
                    continue;
                }
                let d = s.delta(s.m.pos[b], s.stand(b, x));
                // reachable this tick: already there (toggle on if off) or one step while on
                let feasible = d == 0 || (d.abs() == 1 && s.m.active[b]);
                if feasible && pick.map_or(true, |(_, pd)| d.abs() < pd.abs()) {
                    pick = Some((b, d));
                }
            }
            let Some((b, d)) = pick else { strays += 1; continue };
            used.push(b);
            line[b] = if d == 0 { if s.m.active[b] { b'-' } else { b'T' } } else if d > 0 { b'F' } else { b'B' };
        }
        // unblocked extras become strays to erase later: only worth it if the line still pays
        if strays > 0 && (new_cells as f64) * single_cost <= 1.0 + strays as f64 * erase_cost {
            continue;
        }
        for &b in &blockers {
            if !used.contains(&b) && s.m.active[b] {
                line[b] = b'T';
            }
        }
        if !s.valid(&line) {
            continue;
        }
        if best.as_ref().map_or(true, |(_, bn)| new_cells > *bn) {
            best = Some((line, new_cells));
        }
    }
    best
}

/// The first board cell of the line a fill fires, from its first stand.
fn cells_of_stand(s: &Shared, f: &Fill) -> (i32, i32) {
    let (e1, st) = f.stands[0];
    let (dr, dc) = crate::board::direction(e1 as u8 + 1);
    let (r, c) = s.m.ring[st];
    (r + dr, c + dc)
}

/// Travel line for an erase that keeps beams on where harmless: every
/// emitter not yet in place steps once; in-place emitters are switched on.
/// Returns the line and whether all three fire this tick.
pub(crate) fn erase_line_hot(s: &Shared, stands: &[(usize, usize)]) -> ([u8; 6], bool) {
    let mut line = [b'-'; 6];
    let involved: Vec<usize> = stands.iter().map(|&(e, _)| e).collect();
    for e in 0..6 {
        if !involved.contains(&e) && s.m.active[e] {
            line[e] = b'T';
        }
    }
    let mut fire = true;
    for &(e, st) in stands {
        let d = s.delta(s.m.pos[e], st);
        if d != 0 {
            line[e] = if d > 0 { b'F' } else { b'B' };
            if d.abs() > 1 || !s.m.active[e] {
                fire = false;
            }
        } else if !s.m.active[e] {
            line[e] = b'T';
        }
    }
    (line, fire)
}

/// Instructions for one triple only. mode 0 = hot (keep beams on while
/// stepping), 1 = warm (movers switched off, in-place beams kept), 2 = cold
/// (everything off while travelling). Returns (line part, fires this tick).
fn triple_line(s: &Shared, stands: &[(usize, usize)], mode: u8) -> ([u8; 6], bool) {
    let mut line = [b'-'; 6];
    let all_in_place = stands.iter().all(|&(e, st)| s.delta(s.m.pos[e], st) == 0);
    let mut fire = true;
    for &(e, st) in stands {
        let d = s.delta(s.m.pos[e], st);
        let on = s.m.active[e];
        if d == 0 {
            if mode == 2 && !all_in_place {
                if on { line[e] = b'T'; }
                fire = false;
            } else if !on {
                line[e] = b'T';
            }
        } else {
            fire = false;
            match mode {
                0 => {
                    line[e] = if d > 0 { b'F' } else { b'B' };
                    if on && d.abs() == 1 { /* may still fire */ } else { fire = false; }
                }
                _ => {
                    line[e] = if on { b'T' } else if d > 0 { b'F' } else { b'B' };
                }
            }
        }
    }
    if mode == 0 {
        // hot fires only if after the step everyone stands at the target and is on
        fire = stands.iter().all(|&(e, st)| {
            let d = s.delta(s.m.pos[e], st);
            (d == 0 && (s.m.active[e] || line[e] == b'T')) || (d.abs() == 1 && s.m.active[e])
        });
    }
    (line, fire)
}

fn triple_off(s: &Shared, emitters: &[usize]) -> [u8; 6] {
    let mut line = [b'-'; 6];
    for &e in emitters {
        if s.m.active[e] {
            line[e] = b'T';
        }
    }
    line
}

/// One erase: three emitters, one per axis, concurrent at `cell`.
pub(crate) fn erase_plan(s: &Shared, cell: (i32, i32)) -> (Vec<(usize, usize)>, usize) {
    let mut best: Option<(Vec<(usize, usize)>, usize)> = None;
    for &a in &[0usize, 3] {
        for &b in &[1usize, 4] {
            for &c in &[2usize, 5] {
                let stands = vec![(a, s.stand(a, cell)), (b, s.stand(b, cell)), (c, s.stand(c, cell))];
                let f = Fill { stands: stands.clone(), new_cells: 0 };
                let t = s.fill_ticks(&f);
                if best.as_ref().map_or(true, |(_, bt)| t < *bt) {
                    best = Some((stands, t));
                }
            }
        }
    }
    best.unwrap()
}

pub fn build(radius: usize, pattern: &Pattern, count: usize, pairs: &[(usize, usize)], erase_cost: f64, single_cost: f64) -> Vec<[u8; 6]> {
    build_with(radius, pattern, count, pairs, erase_cost, single_cost, None)
}

pub fn build_with(radius: usize, pattern: &Pattern, count: usize, pairs: &[(usize, usize)], erase_cost: f64, single_cost: f64, hub: Option<&dyn Fn(&mut Shared, &Pattern, &mut Vec<[u8; 6]>)>) -> Vec<[u8; 6]> {
    build_mode(radius, pattern, count, pairs, erase_cost, single_cost, hub, None)
}

/// Sweep every line of `axis` with the opposing pair switched on, one tick per
/// line, creating everything on the board; extras are erased afterwards.
fn full_sweep(s: &mut Shared, lines: &mut Vec<[u8; 6]>, axis: usize) {
    let (e1, e2) = AXES[axis];
    let radius = (s.m.ring.len() / 6) as i32;
    // lines of this axis in ring order of e1's stands
    let mut stands: Vec<(usize, usize)> = Vec::new();
    for &c in &s.m.ring.clone() {
        let (dr, dc) = crate::board::direction(e1 as u8 + 1);
        let first = (c.0 + dr, c.1 + dc);
        if matches!(s.m.board.get(first.0, first.1), b'O' | b'M') {
            let cells = s.line_cells(first, e1);
            stands.push((s.stand(e1, cells[0]), s.stand(e2, *cells.last().unwrap())));
        }
    }
    let _ = radius;
    // the entry arc may wrap around ring index 0: rotate so the gap is at the end
    let n = s.m.ring.len();
    let len = stands.len();
    if let Some(gap) = (0..len).find(|&i| (stands[(i + 1) % len].0 + n - stands[i].0) % n != 1) {
        stands.rotate_left((gap + 1) % len);
    }
    // start at whichever end is nearer for e1
    let d_first = s.delta(s.m.pos[e1], stands[0].0).abs();
    let d_last = s.delta(s.m.pos[e1], stands.last().unwrap().0).abs();
    if d_last < d_first {
        stands.reverse();
    }
    // travel to the first line with everything off
    loop {
        let mut line = [b'-'; 6];
        let mut ready = true;
        for e in 0..6 {
            if e != e1 && e != e2 && s.m.active[e] {
                line[e] = b'T';
            }
        }
        for &(e, st) in &[(e1, stands[0].0), (e2, stands[0].1)] {
            let d = s.delta(s.m.pos[e], st);
            if d != 0 {
                ready = false;
                line[e] = if s.m.active[e] { b'T' } else if d > 0 { b'F' } else { b'B' };
            } else if s.m.active[e] && !ready {
                line[e] = b'T';
            }
        }
        if ready {
            for &e in &[e1, e2] {
                if !s.m.active[e] {
                    line[e] = b'T';
                }
            }
        }
        assert!(s.valid(&line), "full sweep travel invalid");
        s.m.tick(&line);
        lines.push(line);
        if ready {
            break;
        }
    }
    for w in stands.windows(2) {
        let mut line = [b'-'; 6];
        let d1 = s.delta(s.m.pos[e1], w[1].0);
        let d2 = s.delta(s.m.pos[e2], w[1].1);
        assert!(d1.abs() == 1 && d2.abs() == 1, "sweep step not adjacent");
        line[e1] = if d1 > 0 { b'F' } else { b'B' };
        line[e2] = if d2 > 0 { b'F' } else { b'B' };
        s.m.tick(&line);
        lines.push(line);
    }
    let mut line = [b'-'; 6];
    line[e1] = b'T';
    line[e2] = b'T';
    s.m.tick(&line);
    lines.push(line);
}

pub fn build_mode(radius: usize, pattern: &Pattern, count: usize, pairs: &[(usize, usize)], erase_cost: f64, single_cost: f64, hub: Option<&dyn Fn(&mut Shared, &Pattern, &mut Vec<[u8; 6]>)>, sweep_axis: Option<usize>) -> Vec<[u8; 6]> {
    let mut s = Shared::new(radius, pattern);
    let mut lines: Vec<[u8; 6]> = Vec::new();
    let mut plans: Vec<PairPlan> = pairs.iter().map(|&(a, b)| PairPlan { a, b, target: None, stalls: 0 }).collect();
    let pair_emitters: Vec<usize> = pairs.iter().flat_map(|&(a, b)| [a, b]).collect();
    let stats = std::env::var("NANO_STATS").is_ok();
    let (mut t_fill, mut t_erase, mut t_single, mut n_fill_cells, mut n_fills, mut n_erase, mut n_single) = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    for _ in 0..count {
        // phase 1: profitable line fills
        s.allow_stray = true;
        if let Some(axis) = sweep_axis {
            full_sweep(&mut s, &mut lines, axis);
            t_fill += lines.len();
        }
        loop {
            let remaining: Vec<(i32, i32)> = pattern.cells.iter().copied().filter(|&(r, c)| s.m.board.get(r, c) == b'O').collect();
            if remaining.is_empty() {
                break;
            }
            // sweep: continue along adjacent lines while the pair is on
            let mut swept = false;
            for &(e1, e2) in &AXES {
                if let Some((line, made)) = sweep_step(&s, e1, e2, erase_cost, single_cost) {
                    s.m.tick(&line);
                    lines.push(line);
                    t_fill += 1;
                    n_fills += 1;
                    n_fill_cells += made;
                    swept = true;
                    break;
                }
            }
            if swept {
                continue;
            }
            let mut seen: HashSet<(usize, (i32, i32))> = HashSet::new();
            let mut best: Option<(Fill, f64)> = None;
            for &cell in &remaining {
                for (axis, &(e1, e2)) in AXES.iter().enumerate() {
                    let cells = s.line_cells(cell, e1);
                    if !seen.insert((axis, cells[0])) {
                        continue;
                    }
                    if let Some((f, cost, new_cells)) = plan_fill(&s, &cells, e1, e2, erase_cost) {
                        let gain = new_cells as f64 * single_cost - cost;
                        if gain > 0.0 && best.as_ref().map_or(true, |(_, g)| gain > *g) {
                            best = Some((f, gain));
                        }
                    }
                }
            }
            let Some((f, _)) = best else { break };
            n_fills += 1;
            n_fill_cells += f.new_cells;
            // lookahead: where should the free emitters go for the best next line on the same axis?
            let busy: Vec<usize> = f.stands.iter().map(|&(e, _)| e).collect();
            let free: Vec<usize> = (0..6).filter(|e| !busy.contains(e)).collect();
            let (fe1, fe2) = (f.stands[0].0, f.stands[1].0);
            let f_line = s.line_cells(cells_of_stand(&s, &f), fe1);
            let mut pre: Vec<(usize, usize)> = Vec::new();
            {
                let mut seen2: HashSet<(i32, i32)> = HashSet::new();
                let mut best2: Option<(Fill, f64)> = None;
                for &cell in &remaining {
                    if f_line.contains(&cell) {
                        continue;
                    }
                    let cells = s.line_cells(cell, fe1);
                    if !seen2.insert(cells[0]) {
                        continue;
                    }
                    if let Some((g, cost, new_cells)) = plan_fill_with(&s, &cells, fe1, fe2, erase_cost, &free) {
                        // prefer adjacent lines: pair travel after this fill
                        let travel = s.delta(f.stands[0].1, g.stands[0].1).abs() as f64;
                        let gain = new_cells as f64 * single_cost - cost - travel;
                        if gain > 0.0 && best2.as_ref().map_or(true, |(_, bg)| gain > *bg) {
                            best2 = Some((g, gain));
                        }
                    }
                }
                if let Some((g, _)) = best2 {
                    pre = g.stands.iter().copied().filter(|(e, _)| free.contains(e)).collect();
                }
            }
            loop {
                let (warm, warm_fired) = s.fill_line_warm(&f);
                let (mut line, fired) = if s.valid(&warm) {
                    (warm, warm_fired)
                } else {
                    let (cold, cold_fired) = s.fill_line(&f);
                    assert!(s.valid(&cold), "fill line invalid");
                    (cold, cold_fired)
                };
                // free emitters pre-position for the next line (switched off)
                for &(e, st) in &pre {
                    if line[e] != b'-' {
                        continue;
                    }
                    let d = s.delta(s.m.pos[e], st);
                    let mv = if s.m.active[e] { b'T' } else if d > 0 { b'F' } else if d < 0 { b'B' } else { b'-' };
                    let mut trial = line;
                    trial[e] = mv;
                    if s.valid(&trial) {
                        line = trial;
                    }
                }
                s.m.tick(&line);
                lines.push(line);
                t_fill += 1;
                if fired {
                    break;
                }
                assert!(lines.len() < 200_000, "runaway fill");
            }
        }
        // phase 2: erase stray matter with two triples working concurrently
        s.allow_stray = false;
        let triples: [[usize; 3]; 2] = [[0, 1, 2], [3, 4, 5]];
        let mut targets: [Option<(i32, i32)>; 2] = [None, None];
        loop {
            let strays: Vec<(i32, i32)> = s.m.board.find(|b| b == b'M').into_iter().filter(|c| !s.pattern.contains(c)).collect();
            if strays.is_empty() {
                break;
            }
            for i in 0..2 {
                if let Some(t) = targets[i] {
                    if !strays.contains(&t) {
                        targets[i] = None;
                    }
                }
            }
            for i in 0..2 {
                if targets[i].is_none() {
                    let other = targets[1 - i];
                    let mut best: Option<((i32, i32), i32)> = None;
                    for &c in &strays {
                        if Some(c) == other && strays.len() > 1 {
                            continue;
                        }
                        let cost = triples[i].iter().map(|&e| s.delta(s.m.pos[e], s.stand(e, c)).abs()).max().unwrap();
                        if best.map_or(true, |(_, bc)| cost < bc) {
                            best = Some((c, cost));
                        }
                    }
                    targets[i] = best.map(|(c, _)| c);
                }
            }
            let stands: Vec<Vec<(usize, usize)>> = (0..2)
                .map(|i| match targets[i] {
                    Some(c) => triples[i].iter().map(|&e| (e, s.stand(e, c))).collect(),
                    None => Vec::new(),
                })
                .collect();
            let mut line = [b'-'; 6];
            let mut fired_any = false;
            for i in 0..2 {
                let mut chosen = None;
                if !stands[i].is_empty() {
                    for mode in 0..3u8 {
                        let (part, fire) = triple_line(&s, &stands[i], mode);
                        let trial = merge(&line, &part);
                        if s.valid(&trial) {
                            chosen = Some((trial, fire));
                            break;
                        }
                    }
                }
                match chosen {
                    Some((trial, fire)) => {
                        line = trial;
                        fired_any |= fire;
                    }
                    None => {
                        let trial = merge(&line, &triple_off(&s, &triples[i]));
                        if s.valid(&trial) {
                            line = trial;
                        }
                    }
                }
            }
            if line == [b'-'; 6] && !fired_any {
                // nothing can move without re-creating an erased cell: go dark, cold mode continues
                for e in 0..6 {
                    if s.m.active[e] {
                        line[e] = b'T';
                    }
                }
            }
            assert!(s.valid(&line), "erase line invalid");
            if lines.len() > 199_980 {
                eprintln!("ERASE STUCK: line {} targets {:?} strays {} pos {:?} active {:?} stands {:?}", std::str::from_utf8(&line).unwrap(), targets, strays.len(), s.m.pos, s.m.active, stands);
            }
            s.m.tick(&line);
            lines.push(line);
            t_erase += 1;
            if fired_any {
                n_erase += 1;
            }
            assert!(lines.len() < 200_000, "runaway erase");
        }
        if let Some(h) = hub {
            h(&mut s, pattern, &mut lines);
        }
        // phase 3: remaining cells one by one, all pairs concurrently
        loop {
            let remaining: Vec<(i32, i32)> = pattern.cells.iter().copied().filter(|&(r, c)| s.m.board.get(r, c) == b'O').collect();
            if remaining.is_empty() {
                break;
            }
            for i in 0..plans.len() {
                if let Some(t) = plans[i].target {
                    if s.m.board.get(t.0, t.1) == b'M' {
                        plans[i].target = None;
                    }
                }
            }
            for i in 0..plans.len() {
                if plans[i].target.is_none() {
                    let others: Vec<(i32, i32)> = plans.iter().enumerate().filter(|(j, _)| *j != i).filter_map(|(_, p)| p.target).collect();
                    let (a, b) = (plans[i].a, plans[i].b);
                    let mut best: Option<((i32, i32), usize)> = None;
                    for &cell in &remaining {
                        if others.contains(&cell) && remaining.len() > others.len() {
                            continue;
                        }
                        let e = s.estimate(cell, a, b);
                        if best.map_or(true, |(_, be)| e < be) {
                            best = Some((cell, e));
                        }
                    }
                    plans[i].target = best.map(|(c, _)| c);
                }
            }
            let mut cands = plans[0].next_line(&s, plans[0].target.unwrap());
            for cand in cands.iter_mut() {
                for e in 0..6 {
                    if !pair_emitters.contains(&e) && s.m.active[e] {
                        cand[e] = b'T';
                    }
                }
            }
            // secondaries: candidate lists and off lines
            let mut sec: Vec<(Vec<[u8; 6]>, [u8; 6])> = Vec::new();
            for i in 1..plans.len() {
                let t = plans[i].target.unwrap();
                let mut c = plans[i].next_line(&s, t);
                c.push(plans[i].next_line_quiet(&s, t));
                sec.push((c, plans[i].off_line(&s)));
            }
            let mut chosen: Option<([u8; 6], Vec<bool>)> = None;
            for l1 in &cands {
                let mut line = *l1;
                let mut stalled = vec![false; plans.len()];
                let mut ok = true;
                for (k, (c, off)) in sec.iter().enumerate() {
                    let mut got = None;
                    for l2 in c {
                        let trial = merge(&line, l2);
                        if s.valid(&trial) {
                            got = Some(trial);
                            break;
                        }
                    }
                    match got {
                        Some(t) => line = t,
                        None => {
                            let trial = merge(&line, off);
                            if s.valid(&trial) {
                                line = trial;
                                stalled[k + 1] = true;
                            } else {
                                ok = false;
                                break;
                            }
                        }
                    }
                }
                if ok && s.valid(&line) {
                    chosen = Some((line, stalled));
                    break;
                }
            }
            let (line, stalled) = chosen.unwrap_or_else(|| {
                // everyone but the primary pair goes dark; the primary's safe candidate keeps <= 1 beam
                let mut line = *cands.last().unwrap();
                for e in 0..6 {
                    if e != plans[0].a && e != plans[0].b && s.m.active[e] {
                        line[e] = b'T';
                    }
                }
                assert!(s.valid(&line), "even the dark fallback is invalid");
                (line, vec![true; plans.len()])
            });
            for i in 1..plans.len() {
                if stalled[i] {
                    plans[i].stalls += 1;
                    if plans[i].stalls > 12 {
                        plans[i].target = None;
                        plans[i].stalls = 0;
                    }
                } else {
                    plans[i].stalls = 0;
                }
            }
            let before = s.m.board.find(|b| b == b'M').len();
            s.m.tick(&line);
            lines.push(line);
            t_single += 1;
            n_single += s.m.board.find(|b| b == b'M').len().saturating_sub(before);
            assert!(lines.len() < 200_000, "runaway program");
        }
        assert!(Pattern::new(pattern.cells.clone()).is_valid(&s.m));
        pattern.remove(&mut s.m);
        for p in plans.iter_mut() {
            p.target = None;
        }
    }
    if stats {
        eprintln!("r={} cells={} x{} mode={:?} ec={} sc={}: fills {} ({} cells, {} ticks) erases {} ({} ticks) singles {} cells ({} ticks) total {}",
            radius, pattern.cells.len(), count, sweep_axis, erase_cost, single_cost, n_fills, n_fill_cells, t_fill, n_erase, t_erase, n_single, t_single, lines.len());
    }
    lines
}

#[allow(dead_code)]
fn unused(_: &Machine) {}
