//! Level 6, greedy multi-beam phase: every tick each emitter may hold, toggle,
//! or step towards the stand of its nearest missing cell. All 3^6 combinations
//! are simulated; the valid one creating the most pattern cells wins, with
//! movement towards work as tie-breaker and toggles slightly penalised. Inside
//! a dense blob this lets up to six beams create several cells per tick.

use crate::level5::Pattern;
use crate::level6b::Shared;

pub fn greedy_phase(s: &mut Shared, pattern: &Pattern, lines: &mut Vec<[u8; 6]>, max_idle: usize) {
    let stats = std::env::var("NANO_STATS").is_ok();
    let (t0, m0) = (lines.len(), s.m.board.find(|b| b == b'M').len());
    let n = s.m.ring.len();
    let mut idle = 0;
    loop {
        let missing: Vec<(i32, i32)> = pattern.cells.iter().copied().filter(|&(r, c)| s.m.board.get(r, c) == b'O').collect();
        if missing.is_empty() {
            break;
        }
        // per emitter: step direction towards the nearest missing cell's stand
        let mut step: [i32; 6] = [0; 6];
        let mut dist: [i32; 6] = [0; 6];
        for e in 0..6 {
            let mut best: Option<i32> = None;
            for &cell in &missing {
                let d = s.delta(s.m.pos[e], s.stand(e, cell));
                if best.map_or(true, |b: i32| d.abs() < b.abs()) {
                    best = Some(d);
                }
            }
            let d = best.unwrap_or(0);
            step[e] = d.signum();
            dist[e] = d.abs();
        }
        let mut best: Option<(i64, [u8; 6])> = None;
        for combo in 0..729usize {
            let mut line = [b'-'; 6];
            let mut pos = s.m.pos;
            let mut active = s.m.active;
            let (mut progress, mut toggles) = (0i64, 0i64);
            let mut c = combo;
            for e in 0..6 {
                let a = c % 3;
                c /= 3;
                match a {
                    1 => {
                        line[e] = b'T';
                        active[e] = !active[e];
                        toggles += 1;
                    }
                    2 => {
                        if step[e] == 0 {
                            continue;
                        }
                        line[e] = if step[e] > 0 { b'F' } else { b'B' };
                        pos[e] = (pos[e] as i32 + step[e]).rem_euclid(n as i32) as usize;
                        progress += 1;
                    }
                    _ => {}
                }
            }
            if let Some(made) = s.creations(&pos, &active) {
                // a lit emitter standing still without creating is wasted, favour movement
                let score = made as i64 * 100 + progress * 3 - toggles * 2;
                if best.map_or(true, |(bs, _)| score > bs) {
                    best = Some((score, line));
                }
            }
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
        lines.push(line);
        let made = s.m.board.find(|b| b == b'M').len() - before;
        idle = if made == 0 { idle + 1 } else { 0 };
        if idle > max_idle {
            break;
        }
        assert!(lines.len() < 300_000, "runaway greedy");
    }
    if stats {
        eprintln!("  greedy phase: {} cells in {} ticks", s.m.board.find(|b| b == b'M').len() - m0, lines.len() - t0);
    }
}
