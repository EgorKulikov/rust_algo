//! Level 6, dual sweeps: two hub beams of one axis parked on two parallel
//! lines, two sweeper beams of a second axis (parallel to each other) stepping
//! along their arc. Hub and sweeper lines meet only at the intended cells, so
//! each sweeper step creates one cell per active hub line. Sweepers switch off
//! across gaps; the second hub is switched on opportunistically whenever both
//! sweeper crossings on its line are pattern cells.

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

/// All lines of `axis` as cell lists, with the number of missing pattern cells.
fn lines_of(s: &Shared, axis: usize) -> Vec<(Vec<(i32, i32)>, usize)> {
    let e1 = AXES[axis].0;
    let (dr, dc) = direction(e1 as u8 + 1);
    let mut out = Vec::new();
    for &c in &s.m.ring {
        let first = (c.0 + dr, c.1 + dc);
        if matches!(s.m.board.get(first.0, first.1), b'O' | b'M') {
            let cells = s.line_cells(first, e1);
            let missing = cells.iter().filter(|&&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').count();
            out.push((cells, missing));
        }
    }
    out
}

/// Diagonal index of a cell along `sweep_axis`: cells on one sweeper line
/// share it. Computed as the ring position of the sweeper's stand.
fn diag_index(s: &Shared, cell: (i32, i32), sweeper: usize) -> usize {
    s.stand(sweeper, cell)
}

pub fn dual_phase(s: &mut Shared, pattern: &Pattern, lines: &mut Vec<[u8; 6]>, hub_axis: usize, sweep_axis: usize, min_missing: usize) {
    let (h1, h2) = AXES[hub_axis];
    let (s1, s2) = AXES[sweep_axis];
    let others: Vec<usize> = (0..6).filter(|&e| ![h1, h2, s1, s2].contains(&e)).collect();
    let stats = std::env::var("NANO_STATS").is_ok();
    let (t0, m0) = (lines.len(), s.m.board.find(|b| b == b'M').len());
    let _ = pattern;
    let n = s.m.ring.len() as i32;
    // rows in arc order, each with its hub-1 stand
    let all_rows: Vec<Vec<(i32, i32)>> = lines_of(s, hub_axis).into_iter().map(|(c, _)| c).collect();
    let mut idle_rounds = 0;
    loop {
        let missing_of = |s: &Shared, row: &Vec<(i32, i32)>| row.iter().filter(|&&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').count();
        // next row: nearest (in hub-1 travel) row with enough missing cells
        let mut best: Option<(usize, i32)> = None;
        for (i, row) in all_rows.iter().enumerate() {
            if missing_of(s, row) < min_missing {
                continue;
            }
            let d = s.delta(s.m.pos[h1], s.stand(h1, row[0])).abs();
            if best.map_or(true, |(_, bd)| d < bd) {
                best = Some((i, d));
            }
        }
        let Some((ri, _)) = best else { break };
        let row1 = all_rows[ri].clone();
        let on1: std::collections::HashSet<(i32, i32)> = row1.iter().copied().collect();
        let hub1_stand = s.stand(h1, row1[0]);
        // row 2: best alignment of pattern cells by diagonal index with row 1's missing cells
        let idx1: std::collections::HashSet<usize> = row1.iter().filter(|&&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').map(|&c| diag_index(s, c, s1)).collect();
        // row 2: the adjacent row with the better alignment of missing cells (blob rows line up)
        let mut row2: Option<Vec<(i32, i32)>> = None;
        let mut best_align = 0usize;
        for j in [ri + 1, ri.wrapping_sub(1)] {
            let Some(row) = all_rows.get(j) else { continue };
            if missing_of(s, row) == 0 {
                continue;
            }
            let align = row.iter().filter(|&&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O' && idx1.contains(&diag_index(s, (r, c), s1))).count();
            if row2.is_none() || align > best_align {
                best_align = align;
                row2 = Some(row.clone());
            }
        }
        let on2: std::collections::HashSet<(i32, i32)> = row2.iter().flatten().copied().collect();
        let hub2_stand = row2.as_ref().map(|r| s.stand(h2, *r.last().unwrap()));
        let made_before = s.m.board.find(|b| b == b'M').len();
        let mut ticks = 0;
        loop {
            let missing1: Vec<(i32, i32)> = row1.iter().copied().filter(|&(r, c)| s.pattern.contains(&(r, c)) && s.m.board.get(r, c) == b'O').collect();
            if missing1.is_empty() {
                break;
            }
            // candidate actions per emitter; evaluate all combinations by simulated creations
            let mut sorted = missing1.clone();
            sorted.sort_by_key(|c| c.1);
            let ends = [Some(sorted[0]), if sorted.len() > 1 { Some(*sorted.last().unwrap()) } else { None }];
            let d1 = s.delta(s.m.pos[h1], hub1_stand);
            let d2 = hub2_stand.map(|st| s.delta(s.m.pos[h2], st));
            let mut options: Vec<Vec<u8>> = vec![Vec::new(); 6];
            for &e in &others {
                options[e] = vec![if s.m.active[e] { b'T' } else { b'-' }];
            }
            // hubs: travel while not in place; in place: on or off (free toggles)
            options[h1] = if d1 != 0 { vec![step_char(d1.signum())] } else { vec![b'-', b'T'] };
            options[h2] = match d2 {
                Some(d) if d != 0 => vec![step_char(d.signum())],
                Some(_) => vec![b'-', b'T'],
                None => vec![if s.m.active[h2] { b'T' } else { b'-' }],
            };
            for (i, &e) in [s1, s2].iter().enumerate() {
                options[e] = match ends[i] {
                    None => vec![if s.m.active[e] { b'T' } else { b'-' }],
                    Some(cell) => {
                        let d = s.delta(s.m.pos[e], s.stand(e, cell));
                        if d == 0 { vec![b'-', b'T'] } else { vec![step_char(d.signum()), b'T'] }
                    }
                };
            }
            let n = s.m.ring.len();
            let mut best: Option<(i64, [u8; 6])> = None;
            let mut idx = [0usize; 6];
            loop {
                let mut line = [b'-'; 6];
                for e in 0..6 {
                    line[e] = options[e][idx[e]];
                }
                let mut pos = s.m.pos;
                let mut active = s.m.active;
                let (mut steps, mut toggles) = (0i64, 0i64);
                for e in 0..6 {
                    match line[e] {
                        b'F' => { pos[e] = (pos[e] + 1) % n; steps += 1; }
                        b'B' => { pos[e] = (pos[e] + n - 1) % n; steps += 1; }
                        b'T' => { active[e] = !active[e]; toggles += 1; }
                        _ => {}
                    }
                }
                if let Some(made) = s.creations(&pos, &active) {
                    // sweepers are only useful lit; hubs idle off cost nothing
                    let score = made as i64 * 10 + steps * 3 - toggles;
                    if best.map_or(true, |(bs, _)| score > bs) {
                        best = Some((score, line));
                    }
                }
                // next combination
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
            let hub1_after = true;
            assert!(s.valid(&line), "dual line invalid");
            let dbg = std::env::var("NANO_DEBUG_DUAL").is_ok() && lines.len() - t0 < 70;
            let before_dbg = s.m.board.find(|b| b == b'M').len();
            s.m.tick(&line);
            if dbg {
                let cr = |e: usize| beam_cells(s, s.m.pos[e], e).into_iter().find(|c| on1.contains(c));
                eprintln!("  t{:>3} {} row1#{} pos={:?} act={:?} made={} cross1=({:?},{:?}) hub1_after={}", lines.len() - t0, std::str::from_utf8(&line).unwrap(), ri, s.m.pos, s.m.active, s.m.board.find(|b| b == b'M').len() - before_dbg, cr(s1), cr(s2), hub1_after);
            }
            lines.push(line);
            ticks += 1;
            if ticks > 30 + 3 * row1.len() {
                break;
            }
            assert!(lines.len() < 300_000, "runaway dual");
        }
        let made = s.m.board.find(|b| b == b'M').len() - made_before;
        if stats {
            eprintln!("    dual row {}: {} cells in {} ticks (row2 {})", ri, made, ticks, row2.is_some());
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
        eprintln!("  dual phase hub {} sweep {}: {} cells in {} ticks", hub_axis, sweep_axis, s.m.board.find(|b| b == b'M').len() - m0, lines.len() - t0);
    }
}
