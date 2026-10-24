//! Level 6, hub sweeps: one emitter beam parked along a hex line (the hub);
//! the four emitters of the other two axes sweep across it, each creating the
//! cell where its beam crosses the hub line, one ring step per tick. A sweeper
//! runs with its beam on across pattern cells, switches off before a gap and
//! on again after it. Each tick is validated as a whole; conflicting sweepers
//! are switched off for that tick. The hub moves on when its line is done.

use crate::board::direction;
use crate::level5::Pattern;
use crate::level6b::Shared;
use crate::level6c::AXES;

/// Cells a beam from ring position `pos` in direction of emitter `e` crosses.
fn beam(s: &Shared, pos: usize, e: usize) -> Vec<(i32, i32)> {
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

/// Lines (by first cell) of `axis` with their missing cells.
fn lines_with_missing(s: &Shared, pattern: &Pattern, axis: usize) -> Vec<(Vec<(i32, i32)>, usize)> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for &cell in &pattern.cells {
        if s.m.board.get(cell.0, cell.1) != b'O' {
            continue;
        }
        let cells = s.line_cells(cell, AXES[axis].0);
        if !seen.insert(cells[0]) {
            continue;
        }
        let missing = cells.iter().filter(|&&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').count();
        out.push((cells, missing));
    }
    out
}

pub fn hub_phase(s: &mut Shared, pattern: &Pattern, lines: &mut Vec<[u8; 6]>, axis: usize, min_missing: usize) {
    let (h1, h2) = AXES[axis];
    let sweepers: Vec<usize> = (0..6).filter(|&e| e != h1 && e != h2).collect();
    let mut idle_rounds = 0;
    let stats = std::env::var("NANO_STATS").is_ok();
    let (t0, m0) = (lines.len(), s.m.board.find(|b| b == b'M').len());
    let mut lines_done = 0usize;
    loop {
        // choose the next hub line: most missing cells, nearest on ties
        let cand = lines_with_missing(s, pattern, axis);
        let mut best: Option<(Vec<(i32, i32)>, usize, usize, i32)> = None; // cells, hub emitter, missing, travel
        for (cells, missing) in cand {
            if missing < min_missing {
                continue;
            }
            for &h in &[h1, h2] {
                let end = if h == h1 { cells[0] } else { *cells.last().unwrap() };
                let travel = s.delta(s.m.pos[h], s.stand(h, end)).abs();
                let better = match &best {
                    None => true,
                    Some((_, _, bm, bt)) => missing > *bm || (missing == *bm && travel < *bt),
                };
                if better {
                    best = Some((cells.clone(), h, missing, travel));
                }
            }
        }
        let Some((line_cells, hub, _, _)) = best else { break };
        let other_hub = if hub == h1 { h2 } else { h1 };
        let hub_stand = s.stand(hub, if hub == h1 { line_cells[0] } else { *line_cells.last().unwrap() });
        let on_line: std::collections::HashSet<(i32, i32)> = line_cells.iter().copied().collect();
        let mut ticks_here = 0;
        let made_before = s.m.board.find(|b| b == b'M').len();
        loop {
            let missing: Vec<(i32, i32)> = line_cells.iter().copied().filter(|&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').collect();
            if missing.is_empty() {
                break;
            }
            let mut line = [b'-'; 6];
            if s.m.active[other_hub] {
                line[other_hub] = b'T';
            }
            let dh = s.delta(s.m.pos[hub], hub_stand);
            let hub_ready = dh == 0 && s.m.active[hub];
            if dh != 0 {
                // relocate: adjacent line may be stepped onto with the beam on (validated below)
                line[hub] = if dh.abs() == 1 && s.m.active[hub] { step_char(dh) } else if s.m.active[hub] { b'T' } else { step_char(dh) };
            } else if !s.m.active[hub] {
                line[hub] = b'T';
            }
            if !s.valid(&line) {
                // hub step with beam on is not harmless: switch it off instead
                if s.m.active[hub] {
                    line[hub] = b'T';
                }
            }
            // sweepers
            let mut claimed: Vec<(i32, i32)> = Vec::new();
            for &e in &sweepers {
                let cur_cross = beam(s, s.m.pos[e], e).into_iter().find(|c| on_line.contains(c));
                let mut target: Option<((i32, i32), i32)> = None;
                for &cell in &missing {
                    if claimed.contains(&cell) {
                        continue;
                    }
                    let d = s.delta(s.m.pos[e], s.stand(e, cell));
                    if target.map_or(true, |(_, td)| d.abs() < td.abs()) {
                        target = Some((cell, d));
                    }
                }
                let proposal: u8 = match target {
                    None => if s.m.active[e] { b'T' } else { b'-' },
                    Some((cell, d)) => {
                        claimed.push(cell);
                        if d == 0 {
                            if s.m.active[e] { b'-' } else { b'T' }
                        } else {
                            let sgn = d.signum();
                            let n = s.m.ring.len();
                            let np = (s.m.pos[e] as i32 + sgn).rem_euclid(n as i32) as usize;
                            let next_cross = beam(s, np, e).into_iter().find(|c| on_line.contains(c));
                            let next_ok = next_cross.map_or(false, |c| s.pattern.contains(&c));
                            let cur_ok = cur_cross.map_or(false, |c| s.pattern.contains(&c));
                            if s.m.active[e] {
                                if next_ok && hub_ready { step_char(sgn) } else { b'T' }
                            } else if cur_ok && hub_ready && cur_cross.map_or(false, |c| s.m.board.get(c.0, c.1) == b'O') {
                                b'T' // light up on a missing cell under the beam
                            } else {
                                step_char(sgn)
                            }
                        }
                    }
                };
                let mut trial = line;
                trial[e] = proposal;
                if s.valid(&trial) {
                    line = trial;
                } else {
                    trial = line;
                    trial[e] = if s.m.active[e] { b'T' } else { b'-' };
                    if s.valid(&trial) {
                        line = trial;
                    }
                }
            }
            if !s.valid(&line) {
                line = [b'-'; 6];
                for e in 0..6 {
                    if s.m.active[e] {
                        line[e] = b'T';
                    }
                }
            }
            assert!(s.valid(&line), "hub line invalid");
            s.m.tick(&line);
            lines.push(line);
            ticks_here += 1;
            assert!(lines.len() < 300_000, "runaway hub");
            if ticks_here > 60 + 4 * line_cells.len() {
                break; // give up on this line
            }
        }
        let made = s.m.board.find(|b| b == b'M').len() - made_before;
        lines_done += 1;
        if stats {
            eprintln!("    hub line len {} : {} cells in {} ticks", line_cells.len(), made, ticks_here);
        }
        if made == 0 {
            idle_rounds += 1;
            if idle_rounds > 2 {
                break;
            }
        } else {
            idle_rounds = 0;
        }
    }
    if stats {
        eprintln!("  hub phase axis {}: {} lines, {} cells in {} ticks", axis, lines_done, s.m.board.find(|b| b == b'M').len() - m0, lines.len() - t0);
    }
}
