//! Level 6 hybrid for dense patterns: an opposing emitter pair sweeps every
//! line through the pattern (one full line per tick), then the beam search
//! (level6h) erases the strays and fixes the rest. Repeated per copy.

use crate::board::direction;
use crate::level4::Machine;
use crate::level5::Pattern;
use crate::level6h::{beam_build_from, Params};

/// Cells crossed by emitter k firing from ring index p, and the ring index where the beam exits.
fn beam(m: &Machine, k: usize, p: usize) -> (Vec<(i32, i32)>, Option<usize>) {
    let (dr, dc) = direction(k as u8 + 1);
    let (mut r, mut c) = m.ring[p];
    r += dr;
    c += dc;
    let mut cells = Vec::new();
    while matches!(m.board.get(r, c), b'O' | b'M') {
        cells.push((r, c));
        r += dr;
        c += dc;
    }
    let exit = m.ring.iter().position(|&x| x == (r, c));
    (cells, exit)
}

fn ring_dist(n: usize, a: usize, b: usize) -> usize {
    let d = (a as i64 - b as i64).unsigned_abs() as usize;
    d.min(n - d)
}

/// Move emitter k from `from` toward `to` by one step; returns the instruction.
fn step_toward(n: usize, from: usize, to: usize) -> u8 {
    if from == to {
        return b'-';
    }
    let fwd = (to + n - from) % n;
    if fwd <= n / 2 {
        b'F'
    } else {
        b'B'
    }
}

fn apply(m: &mut Machine, pattern: &Pattern, line: [u8; 6], lines: &mut Vec<[u8; 6]>, reps: &mut usize) {
    m.tick(&line);
    lines.push(line);
    if pattern.is_valid(m) {
        pattern.remove(m);
        *reps += 1;
    }
}

pub fn hybrid_build(radius: usize, pattern: &Pattern, count: usize, prm: Params, max_ticks: usize) -> Option<Vec<[u8; 6]>> {
    let mut best: Option<Vec<[u8; 6]>> = None;
    for axis in 0..3 {
        if let Some(lines) = hybrid_axis(radius, pattern, count, prm, best.as_ref().map_or(max_ticks, |b| b.len() - 1), axis) {
            if std::env::var("NANO_STATS").is_ok() {
                eprintln!("hybrid axis {}: {} ticks (cells {})", axis, lines.len(), pattern.cells.len());
            }
            if best.as_ref().map_or(true, |b| lines.len() < b.len()) {
                best = Some(lines);
            }
        }
    }
    best
}

fn hybrid_axis(radius: usize, pattern: &Pattern, count: usize, prm: Params, max_ticks: usize, axis: usize) -> Option<Vec<[u8; 6]>> {
    let max_ticks = if std::env::var("NANO_HYBRID_FORCE").is_ok() { 100_000 } else { max_ticks };
    let mut m = Machine::new(radius);
    let n = m.ring.len();
    let k = axis; // emitter k and k+3 are an opposing pair
    let k2 = axis + 3;
    // lines of this axis that contain pattern cells, as (p, q) ring indices
    let mut lines_pq: Vec<(usize, usize)> = Vec::new();
    for p in 0..n {
        let (cells, exit) = beam(&m, k, p);
        if cells.is_empty() {
            continue;
        }
        if !cells.iter().any(|c| pattern.cells.contains(c)) {
            continue;
        }
        let q = match exit { Some(q) => q, None => { if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail exit", axis); } return None; } };
        // the opposite emitter at q must retrace the same cells
        let (back, _) = beam(&m, k2, q);
        if back.len() != cells.len() {
            if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 1", axis); }
            return None;
        }
        lines_pq.push((p, q));
    }
    if lines_pq.is_empty() {
        if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 2", axis); }
        return None;
    }
    // sort along the ring so consecutive lines are consecutive positions
    lines_pq.sort();
    // the p's may wrap around the ring (e.g. left side + top row): rotate so that gaps are at most 1
    let mut start_i = 0;
    for i in 0..lines_pq.len() {
        let prev = lines_pq[(i + lines_pq.len() - 1) % lines_pq.len()].0;
        if (lines_pq[i].0 + n - prev) % n != 1 {
            start_i = i;
        }
    }
    lines_pq.rotate_left(start_i);
    for w in lines_pq.windows(2) {
        if (w[1].0 + n - w[0].0) % n != 1 || ring_dist(n, w[0].1, w[1].1) != 1 {
            if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 3", axis); }
            return None;
        }
    }
    let mut out: Vec<[u8; 6]> = Vec::new();
    let mut reps = 0usize;
    for _rep in 0..count {
        // choose sweep order: start at the end nearer to emitter k
        let (first, last) = (lines_pq[0], *lines_pq.last().unwrap());
        let forward = ring_dist(n, m.pos[k], first.0) <= ring_dist(n, m.pos[k], last.0);
        let order: Vec<(usize, usize)> = if forward { lines_pq.clone() } else { lines_pq.iter().rev().copied().collect() };
        // switch everything off that is on (one tick), travel, toggle on, sweep
        let (p0, q0) = order[0];
        loop {
            let mut line = [b'-'; 6];
            let mut moving = false;
            for e in 0..6 {
                if m.active[e] {
                    line[e] = b'T';
                    moving = true;
                }
            }
            if !moving {
                break;
            }
            apply(&mut m, pattern, line, &mut out, &mut reps);
            if out.len() >= max_ticks {
                if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 4", axis); }
                return None;
            }
        }
        // halo cells (non-pattern cells adjacent to the pattern)
        let halo: Vec<(i32, i32)> = {
            let pset: std::collections::HashSet<(i32, i32)> = pattern.cells.iter().copied().collect();
            let mut h = std::collections::HashSet::new();
            for &c in &pattern.cells {
                for nb in crate::level5::neighbours(c) {
                    if !pset.contains(&nb) && m.board.get(nb.0, nb.1) != b'*' && m.board.get(nb.0, nb.1) != b' ' {
                        h.insert(nb);
                    }
                }
            }
            h.into_iter().collect()
        };
        let pset: std::collections::HashSet<(i32, i32)> = pattern.cells.iter().copied().collect();
        let hset: std::collections::HashSet<(i32, i32)> = halo.iter().copied().collect();
        let others: Vec<usize> = (0..6).filter(|&e| e != k && e != k2).collect();
        // where_[e][cell]: ring index from which emitter e crosses `cell`
        let mut where_: std::collections::HashMap<(usize, (i32, i32)), usize> = std::collections::HashMap::new();
        for e in 0..6 {
            for p in 0..n {
                for c in beam(&m, e, p).0 {
                    where_.insert((e, c), p);
                }
            }
        }
        let eval = |mm: &Machine| -> i64 {
            let mut sc = 0i64;
            for &c in &pattern.cells {
                if mm.board.get(c.0, c.1) == b'M' {
                    sc += 1000;
                }
            }
            for &c in &halo {
                if mm.board.get(c.0, c.1) == b'M' {
                    sc -= 1500;
                }
            }
            sc
        };
        // potential: blockers near the halo cells of the upcoming line
        let potential = |mm: &Machine, next_line: Option<(usize, usize)>| -> i64 {
            let mut pot = 0i64;
            if let Some((p, _)) = next_line {
                let cells = beam(mm, k, p).0;
                let targets: Vec<(i32, i32)> = cells.iter().copied().filter(|c| hset.contains(c)).collect();
                if targets.is_empty() {
                    return 0;
                }
                for &e in &others {
                    let mut best = n as i64;
                    for t in &targets {
                        if let Some(&q) = where_.get(&(e, *t)) {
                            best = best.min(ring_dist(n, mm.pos[e], q) as i64);
                        }
                    }
                    pot += best;
                }
            }
            pot
        };
        let instrs = [b'-', b'F', b'B', b'T'];
        // one tick: fixed instructions for the pair, best of 4^4 for the others
        let mut do_tick = |m: &mut Machine, base: [u8; 6], next_line: Option<(usize, usize)>, out: &mut Vec<[u8; 6]>, reps: &mut usize| {
            let mut best_line = base;
            let mut best_score = i64::MIN;
            for combo in 0..256u32 {
                let mut line = base;
                for (j, &e) in others.iter().enumerate() {
                    line[e] = instrs[((combo >> (2 * j)) & 3) as usize];
                }
                let mut mm = m.clone();
                mm.tick(&line);
                let sc = eval(&mm) * 4 - potential(&mm, next_line);
                if sc > best_score {
                    best_score = sc;
                    best_line = line;
                }
            }
            apply(m, pattern, best_line, out, reps);
        };
        let mut idx = 0usize;
        while m.pos[k] != p0 || m.pos[k2] != q0 {
            let mut line = [b'-'; 6];
            line[k] = step_toward(n, m.pos[k], p0);
            line[k2] = step_toward(n, m.pos[k2], q0);
            do_tick(&mut m, line, Some(order[0]), &mut out, &mut reps);
            if out.len() >= max_ticks {
                if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 5", axis); }
                return None;
            }
        }
        let mut line = [b'-'; 6];
        line[k] = b'T';
        line[k2] = b'T';
        do_tick(&mut m, line, order.get(1).copied(), &mut out, &mut reps);
        for w in order.windows(2) {
            idx += 1;
            let mut line = [b'-'; 6];
            line[k] = step_toward(n, w[0].0, w[1].0);
            line[k2] = step_toward(n, w[0].1, w[1].1);
            do_tick(&mut m, line, order.get(idx + 1).copied(), &mut out, &mut reps);
            if out.len() >= max_ticks {
                if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 6", axis); }
                return None;
            }
        }
        if reps > _rep {
            continue; // pattern already complete (no strays)
        }
        if std::env::var("NANO_STATS").is_ok() {
            let missing = pattern.cells.iter().filter(|c| m.board.get(c.0, c.1) != b'M').count();
            let strays = halo.iter().filter(|c| m.board.get(c.0, c.1) == b'M').count();
            eprintln!("hybrid axis {} after sweep: {} ticks, missing {}, strays {} (halo {})", axis, out.len(), missing, strays, halo.len());
            if std::env::var("NANO_HYBRID_BOARD").is_ok() {
                let mut shown = m.board.clone();
                for &c in &halo {
                    if m.board.get(c.0, c.1) == b'M' {
                        shown.set(c.0, c.1, b'S');
                    } else {
                        shown.set(c.0, c.1, b'h');
                    }
                }
                for &c in &pattern.cells {
                    if m.board.get(c.0, c.1) != b'M' {
                        shown.set(c.0, c.1, b'X');
                    }
                }
                eprint!("{}", shown.render());
            }
        }
        // endgame: beam search from here for one copy
        let budget = max_ticks.saturating_sub(out.len());
        if budget == 0 {
            if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 7", axis); }
            return None;
        }
        let tail = match beam_build_from(radius, pattern, 1, prm, budget, Some(&m)) { Some(t) => t, None => { if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail endgame after {} ticks budget {}", axis, out.len(), budget); } return None; } };
        for line in tail {
            apply(&mut m, pattern, line, &mut out, &mut reps);
        }
        if reps != _rep + 1 {
            if std::env::var("NANO_STATS").is_ok() { eprintln!("hybrid axis {} bail 8", axis); }
            return None;
        }
    }
    Some(out)
}
