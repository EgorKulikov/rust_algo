//! Level 6, three hub-and-sweeper pairs: for each axis i the hub emitter of
//! axis i parks on the line of that axis holding the most missing cells, and
//! the sweeper (an emitter of axis i+1) runs that line end to end. All six
//! emitters work at once; each tick the combination of per-emitter actions
//! (hold, toggle, step) with the most simulated creations is taken, so bonus
//! crossings inside the blob count and stray crossings are avoided.

use crate::board::direction;
use crate::level5::Pattern;
use crate::level6b::Shared;
use crate::level6c::AXES;

fn beam_cells(s: &Shared, pos: usize, e: usize) -> Vec<(i32, i32)> {
    let (dr, dc) = direction(e as u8 + 1);
    let (mut r, mut c) = s.m.ring[pos];
    r += dr;
    c += dc;
    let mut out = Vec::new();
    while matches!(s.m.board.get(r, c), b'O' | b'M') {
        out.push((r, c));
        r += dr;
        c += dc;
    }
    out
}

fn step_char(d: i32) -> u8 {
    if d > 0 { b'F' } else if d < 0 { b'B' } else { b'-' }
}

fn lines_of(s: &Shared, axis: usize) -> Vec<Vec<(i32, i32)>> {
    let e1 = AXES[axis].0;
    let (dr, dc) = direction(e1 as u8 + 1);
    let mut out = Vec::new();
    for &c in &s.m.ring {
        let first = (c.0 + dr, c.1 + dc);
        if matches!(s.m.board.get(first.0, first.1), b'O' | b'M') {
            out.push(s.line_cells(first, e1));
        }
    }
    out
}

struct Pair {
    hub: usize,
    sweeper: usize,
    axis: usize,
    line: Option<Vec<(i32, i32)>>,
    hub_stand: usize,
}

pub fn triple_phase(s: &mut Shared, pattern: &Pattern, lines: &mut Vec<[u8; 6]>, rotation: usize, max_idle: usize) {
    let stats = std::env::var("NANO_STATS").is_ok();
    let (t0, m0) = (lines.len(), s.m.board.find(|b| b == b'M').len());
    let n = s.m.ring.len();
    // hub of axis i with a sweeper from axis i+1 (rotation picks which emitter of each axis is the hub)
    let mut pairs: Vec<Pair> = (0..3)
        .map(|i| {
            let hub = if (rotation >> i) & 1 == 0 { AXES[i].0 } else { AXES[i].1 };
            let j = (i + 1) % 3;
            let sweeper = if (rotation >> i) & 1 == 0 { AXES[j].1 } else { AXES[j].0 };
            Pair { hub, sweeper, axis: i, line: None, hub_stand: 0 }
        })
        .collect();
    let all_lines: Vec<Vec<Vec<(i32, i32)>>> = (0..3).map(|a| lines_of(s, a)).collect();
    let missing_on = |s: &Shared, line: &Vec<(i32, i32)>| -> Vec<(i32, i32)> {
        line.iter().copied().filter(|&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').collect()
    };
    let mut idle = 0;
    loop {
        let missing: Vec<(i32, i32)> = pattern.cells.iter().copied().filter(|&(r, c)| s.m.board.get(r, c) == b'O').collect();
        if missing.is_empty() {
            break;
        }
        // (re)assign lines: a pair whose line is done takes the nearest line with the most missing cells
        let taken: Vec<Option<Vec<(i32, i32)>>> = pairs.iter().map(|p| p.line.clone()).collect();
        for (i, p) in pairs.iter_mut().enumerate() {
            if p.line.as_ref().map_or(true, |l| missing_on(s, l).is_empty()) {
                let mut best: Option<(&Vec<(i32, i32)>, i64)> = None;
                for line in &all_lines[p.axis] {
                    let m = missing_on(s, line).len() as i64;
                    if m == 0 || taken.iter().enumerate().any(|(j, t)| j != i && t.as_ref() == Some(line)) {
                        continue;
                    }
                    let travel = s.delta(s.m.pos[p.hub], s.stand(p.hub, line[0])).abs() as i64;
                    let miss = missing_on(s, line);
                    let sweeper_travel = miss.iter().map(|&c| s.delta(s.m.pos[p.sweeper], s.stand(p.sweeper, c)).abs() as i64).min().unwrap_or(0);
                    let score = m * 4 - travel - sweeper_travel;
                    if best.map_or(true, |(_, bs)| score > bs) {
                        best = Some((line, score));
                    }
                }
                match best {
                    Some((line, _)) => {
                        p.hub_stand = s.stand(p.hub, line[0]);
                        p.line = Some(line.clone());
                    }
                    None => p.line = None,
                }
            }
        }
        // options per emitter
        let mut options: Vec<Vec<u8>> = vec![vec![b'-']; 6];
        for p in &pairs {
            match &p.line {
                None => {
                    options[p.hub] = vec![if s.m.active[p.hub] { b'T' } else { b'-' }];
                    options[p.sweeper] = vec![if s.m.active[p.sweeper] { b'T' } else { b'-' }];
                }
                Some(line) => {
                    let d = s.delta(s.m.pos[p.hub], p.hub_stand);
                    options[p.hub] = if d != 0 { vec![step_char(d.signum()), b'T'] } else { vec![b'-', b'T'] };
                    // sweeper target: nearest missing cell on the line (by stand distance), then monotone
                    let miss = missing_on(s, line);
                    let mut best: Option<i32> = None;
                    for &cell in &miss {
                        let dd = s.delta(s.m.pos[p.sweeper], s.stand(p.sweeper, cell));
                        if best.map_or(true, |b: i32| dd.abs() < b.abs()) {
                            best = Some(dd);
                        }
                    }
                    let dd = best.unwrap_or(0);
                    options[p.sweeper] = if dd != 0 { vec![step_char(dd.signum()), b'T'] } else { vec![b'-', b'T'] };
                }
            }
        }
        let mut best: Option<(i64, [u8; 6])> = None;
        let mut idx = [0usize; 6];
        loop {
            let mut line = [b'-'; 6];
            let mut pos = s.m.pos;
            let mut active = s.m.active;
            let (mut steps, mut toggles) = (0i64, 0i64);
            for e in 0..6 {
                line[e] = options[e][idx[e]];
                match line[e] {
                    b'F' => { pos[e] = (pos[e] + 1) % n; steps += 1; }
                    b'B' => { pos[e] = (pos[e] + n - 1) % n; steps += 1; }
                    b'T' => { active[e] = !active[e]; toggles += 1; }
                    _ => {}
                }
            }
            if let Some(made) = s.creations(&pos, &active) {
                let score = made as i64 * 100 + steps * 3 - toggles * 2;
                if best.map_or(true, |(bs, _)| score > bs) {
                    best = Some((score, line));
                }
            }
            let mut e = 0;
            loop {
                if e == 6 { break; }
                idx[e] += 1;
                if idx[e] < options[e].len() { break; }
                idx[e] = 0;
                e += 1;
            }
            if e == 6 { break; }
        }
        let line = match best {
            Some((_, l)) => l,
            None => {
                let mut l = [b'-'; 6];
                for e in 0..6 {
                    if s.m.active[e] {
                        l[e] = b'T';
                    }
                }
                l
            }
        };
        let before = s.m.board.find(|b| b == b'M').len();
        s.m.tick(&line);
        let made = s.m.board.find(|b| b == b'M').len() - before;
        if std::env::var("NANO_DEBUG_TRIPLE").is_ok() && lines.len() - t0 < 40 {
            eprintln!("  t{:>3} {} made={} pos={:?} act={:?} lines={:?}", lines.len() - t0, std::str::from_utf8(&line).unwrap(), made, s.m.pos, s.m.active, pairs.iter().map(|p| p.line.as_ref().map(|l| (l[0], missing_on(s, l).len()))).collect::<Vec<_>>());
        }
        lines.push(line);
        idle = if made == 0 { idle + 1 } else { 0 };
        if idle > max_idle {
            break;
        }
        assert!(lines.len() < 300_000, "runaway triple");
    }
    let _ = beam_cells;
    if stats {
        eprintln!("  triple phase rot {}: {} cells in {} ticks", rotation, s.m.board.find(|b| b == b'M').len() - m0, lines.len() - t0);
    }
}
