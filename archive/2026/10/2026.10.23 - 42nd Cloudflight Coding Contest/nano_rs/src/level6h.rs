//! Level 6, beam search over machine ticks. State = emitter positions +
//! on/off + matter on the pattern cells and their halo (anything else is
//! irrelevant for validity). Each tick tries all 4^6 instruction combos.

use crate::level4::Machine;
use crate::level5::{neighbours, Pattern};
use std::collections::HashSet;

#[derive(Clone)]
struct State {
    pos: [u8; 6],
    on: u8,
    pat: [u64; W],  // bitmap over pattern cells
    halo: [u64; W], // bitmap over halo cells
    reps: u32,
    score: i64,
    parent: u32,
    instr: [u8; 6],
}

struct Geo {
    n: usize,
    npat: usize,
    /// beams[k][p]: (pattern cell indices, halo cell indices) crossed by emitter k firing from ring index p.
    beams: Vec<Vec<(Vec<u16>, Vec<u16>)>>,
    /// where[k][c]: ring index from which emitter k's beam crosses pattern cell c.
    where_: Vec<Vec<u16>>,
    /// hwhere[k][h]: ring index from which emitter k's beam crosses halo cell h.
    hwhere: Vec<Vec<u16>>,
}

const W: usize = 5;
fn bits(n: usize) -> [u64; W] {
    assert!(n <= 64 * W, "too many cells for beam search");
    [0; W]
}
fn getb(v: &[u64], i: usize) -> bool {
    v[i / 64] >> (i % 64) & 1 == 1
}
fn setb(v: &mut [u64], i: usize, b: bool) {
    if b {
        v[i / 64] |= 1 << (i % 64);
    } else {
        v[i / 64] &= !(1 << (i % 64));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub width: usize,
    pub stray_pen: i64,
    pub pot_w: i64,
    pub cell_w: i64,
    /// max states kept per identical (board, reps); 0 = unlimited
    pub cap: usize,
    /// steer emitters through a precomputed list of blocked line fills
    pub guide: bool,
}

/// One line fill with blockers: fires an opposing pair along a line while the
/// other emitters put a third beam on every halo cell of that line.
struct Op {
    pat_mask: [u64; W],
    halo_mask: [u64; W],
    targets: Vec<(usize, u16)>,
}

fn rdist(n: usize, a: usize, b: usize) -> i64 {
    let d = (a as i64 - b as i64).abs();
    d.min(n as i64 - d)
}

fn build_guide(geo: &Geo, start_pos: &[u8; 6]) -> Vec<Op> {
    let n = geo.n;
    // candidates: (k, p, q, pattern cells, halo cells)
    let mut cands: Vec<(usize, usize, usize, Vec<u16>, Vec<u16>)> = Vec::new();
    for k in 0..3 {
        for p in 0..n {
            let (pc, hc) = &geo.beams[k][p];
            if pc.is_empty() || hc.len() > 4 {
                continue;
            }
            let q = geo.where_[k + 3][pc[0] as usize];
            if q == u16::MAX {
                continue;
            }
            cands.push((k, p, q as usize, pc.clone(), hc.clone()));
        }
    }
    let mut covered = vec![false; geo.npat];
    let mut chosen: Vec<usize> = Vec::new();
    loop {
        let mut best = (0usize, usize::MAX, usize::MAX);
        for (i, c) in cands.iter().enumerate() {
            if chosen.contains(&i) {
                continue;
            }
            let gain = c.3.iter().filter(|&&x| !covered[x as usize]).count();
            if gain > best.0 || (gain == best.0 && gain > 0 && c.4.len() < best.1) {
                best = (gain, c.4.len(), i);
            }
        }
        if best.0 == 0 {
            break;
        }
        for &x in &cands[best.2].3 {
            covered[x as usize] = true;
        }
        chosen.push(best.2);
    }
    // order: by axis (largest group first), sweeping along the ring, alternating direction
    let mut groups: Vec<Vec<usize>> = vec![Vec::new(); 3];
    for &i in &chosen {
        groups[cands[i].0].push(i);
    }
    groups.sort_by_key(|g| std::cmp::Reverse(g.len()));
    let mut order: Vec<usize> = Vec::new();
    for (gi, g) in groups.iter_mut().enumerate() {
        g.sort_by_key(|&i| cands[i].1);
        if gi % 2 == 1 {
            g.reverse();
        }
        order.extend(g.iter().copied());
    }
    // blocker assignment, keeping blockers close to where they were last sent
    let mut last: [usize; 6] = [0; 6];
    for e in 0..6 {
        last[e] = start_pos[e] as usize;
    }
    let mut ops = Vec::new();
    for &i in &order {
        let (k, p, q, pc, hc) = &cands[i];
        let others: Vec<usize> = (0..6).filter(|&e| e != *k && e != k + 3).collect();
        let m = hc.len();
        let mut best_cost = i64::MAX;
        let mut best_assign: Vec<(usize, u16)> = Vec::new();
        // enumerate injective maps hc -> others
        let mut perm = vec![0usize; m];
        fn rec(idx: usize, m: usize, perm: &mut Vec<usize>, used: &mut [bool; 4], others: &[usize], hc: &[u16], geo: &Geo, last: &[usize; 6], n: usize, best_cost: &mut i64, best_assign: &mut Vec<(usize, u16)>) {
            if idx == m {
                let mut cost = 0i64;
                let mut assign = Vec::with_capacity(m);
                for j in 0..m {
                    let e = others[perm[j]];
                    let pos = geo.hwhere[e][hc[j] as usize];
                    if pos == u16::MAX {
                        return;
                    }
                    cost += rdist(n, last[e], pos as usize);
                    assign.push((e, pos));
                }
                if cost < *best_cost {
                    *best_cost = cost;
                    *best_assign = assign;
                }
                return;
            }
            for c in 0..4 {
                if !used[c] {
                    used[c] = true;
                    perm[idx] = c;
                    rec(idx + 1, m, perm, used, others, hc, geo, last, n, best_cost, best_assign);
                    used[c] = false;
                }
            }
        }
        let mut used = [false; 4];
        rec(0, m, &mut perm, &mut used, &others, hc, geo, &last, n, &mut best_cost, &mut best_assign);
        if best_cost == i64::MAX {
            continue;
        }
        let mut targets = vec![(*k, *p as u16), (k + 3, *q as u16)];
        targets.extend(best_assign.iter().copied());
        for &(e, pos) in &targets {
            last[e] = pos as usize;
        }
        let mut pat_mask = [0u64; W];
        let mut halo_mask = [0u64; W];
        for &x in pc {
            setb(&mut pat_mask, x as usize, true);
        }
        for &x in hc {
            setb(&mut halo_mask, x as usize, true);
        }
        ops.push(Op { pat_mask, halo_mask, targets });
    }
    if std::env::var("NANO_STATS").is_ok() {
        eprintln!("guide: {} ops covering {} of {} cells", ops.len(), covered.iter().filter(|&&c| c).count(), geo.npat);
    }
    ops
}

/// Keep the best `width` states, at most `cap` per identical board.
fn prune(next: &mut Vec<State>, width: usize, cap: usize) {
    if cap == 0 {
        if next.len() > width {
            next.select_nth_unstable_by(width, |a, b| b.score.cmp(&a.score));
            next.truncate(width);
        }
        return;
    }
    next.sort_unstable_by(|a, b| b.score.cmp(&a.score));
    let mut counts: std::collections::HashMap<([u64; W], [u64; W], u32), usize> = std::collections::HashMap::new();
    let mut kept = Vec::with_capacity(width);
    for st in next.drain(..) {
        if kept.len() >= width {
            break;
        }
        let c = counts.entry((st.pat, st.halo, st.reps)).or_insert(0);
        if *c < cap {
            *c += 1;
            kept.push(st);
        }
    }
    *next = kept;
}

pub fn beam_build(radius: usize, pattern: &Pattern, count: usize, prm: Params, max_ticks: usize) -> Option<Vec<[u8; 6]>> {
    beam_build_from(radius, pattern, count, prm, max_ticks, None)
}

/// Like `beam_build`, but optionally starting from an existing machine state.
pub fn beam_build_from(radius: usize, pattern: &Pattern, count: usize, prm: Params, max_ticks: usize, start: Option<&Machine>) -> Option<Vec<[u8; 6]>> {
    let width = prm.width;
    let m = Machine::new(radius);
    let n = m.ring.len();
    let pset: HashSet<(i32, i32)> = pattern.cells.iter().copied().collect();
    let mut halo: Vec<(i32, i32)> = Vec::new();
    let mut hset: HashSet<(i32, i32)> = HashSet::new();
    if pattern.cells.len() > 64 * W {
        return None;
    }
    for &c in &pattern.cells {
        for nb in neighbours(c) {
            if !pset.contains(&nb) && m.board.get(nb.0, nb.1) == b'O' && hset.insert(nb) {
                halo.push(nb);
            }
        }
    }
    if halo.len() > 64 * W {
        return None;
    }
    let pidx = |c: (i32, i32)| pattern.cells.iter().position(|&x| x == c);
    let hidx = |c: (i32, i32)| halo.iter().position(|&x| x == c);
    let mut beams = vec![vec![(Vec::new(), Vec::new()); n]; 6];
    let mut where_ = vec![vec![u16::MAX; pattern.cells.len()]; 6];
    let mut hwhere = vec![vec![u16::MAX; halo.len()]; 6];
    for k in 0..6 {
        let (dr, dc) = crate::board::direction(k as u8 + 1);
        for p in 0..n {
            let (mut r, mut c) = m.ring[p];
            r += dr;
            c += dc;
            while matches!(m.board.get(r, c), b'O' | b'M') {
                if let Some(i) = pidx((r, c)) {
                    beams[k][p].0.push(i as u16);
                    where_[k][i] = p as u16;
                } else if let Some(i) = hidx((r, c)) {
                    beams[k][p].1.push(i as u16);
                    hwhere[k][i] = p as u16;
                }
                r += dr;
                c += dc;
            }
        }
    }
    let geo = Geo { n, npat: pattern.cells.len(), beams, where_, hwhere };
    let npat = geo.npat;
    let nhalo = halo.len();
    let mut init = State {
        pos: [m.pos[0] as u8, m.pos[1] as u8, m.pos[2] as u8, m.pos[3] as u8, m.pos[4] as u8, m.pos[5] as u8],
        on: 0,
        pat: bits(npat),
        halo: bits(nhalo),
        reps: 0,
        score: 0,
        parent: u32::MAX,
        instr: [b'-'; 6],
    };
    if let Some(sm) = start {
        for k in 0..6 {
            init.pos[k] = sm.pos[k] as u8;
            if sm.active[k] {
                init.on |= 1 << k;
            }
        }
        for (i, &(r, c)) in pattern.cells.iter().enumerate() {
            setb(&mut init.pat, i, sm.board.get(r, c) == b'M');
        }
        for (i, &(r, c)) in halo.iter().enumerate() {
            setb(&mut init.halo, i, sm.board.get(r, c) == b'M');
        }
    }
    let guide: Option<Vec<Op>> = if prm.guide { Some(build_guide(&geo, &init.pos)) } else { None };
    let mut layers: Vec<Vec<State>> = vec![vec![init]];
    let mut pcount = vec![0u32; npat];
    let mut hcount = vec![0u32; nhalo];
    let mut touched_p: Vec<u16> = Vec::new();
    let mut touched_h: Vec<u16> = Vec::new();
    let instrs = [b'-', b'F', b'B', b'T'];
    for tick in 0..max_ticks {
        let prev = layers.last().unwrap();
        let mut next: Vec<State> = Vec::with_capacity(8 * width + 1);
        let mut seen: HashSet<(u64, u64)> = HashSet::new();
        for (pi, s) in prev.iter().enumerate() {
            let mut missing: Vec<usize> = (0..npat).filter(|&i| !getb(&s.pat, i)).collect();
            if missing.is_empty() {
                missing = (0..npat).collect();
            }
            for combo in 0..4096u32 {
                let mut pos = s.pos;
                let mut on = s.on;
                let mut instr = [b'-'; 6];
                for k in 0..6 {
                    let a = (combo >> (2 * k)) & 3;
                    instr[k] = instrs[a as usize];
                    match a {
                        1 => pos[k] = ((pos[k] as usize + 1) % n) as u8,
                        2 => pos[k] = ((pos[k] as usize + n - 1) % n) as u8,
                        3 => on ^= 1 << k,
                        _ => {}
                    }
                }
                // fire
                touched_p.clear();
                touched_h.clear();
                for k in 0..6 {
                    if on >> k & 1 == 0 {
                        continue;
                    }
                    let (ref pc, ref hc) = geo.beams[k][pos[k] as usize];
                    for &i in pc {
                        if pcount[i as usize] == 0 {
                            touched_p.push(i);
                        }
                        pcount[i as usize] += 1;
                    }
                    for &i in hc {
                        if hcount[i as usize] == 0 {
                            touched_h.push(i);
                        }
                        hcount[i as usize] += 1;
                    }
                }
                let mut pat = s.pat;
                let mut halo = s.halo;
                for &i in &touched_p {
                    let c = pcount[i as usize];
                    pcount[i as usize] = 0;
                    if c == 2 {
                        setb(&mut pat, i as usize, true);
                    } else if c >= 3 {
                        setb(&mut pat, i as usize, false);
                    }
                }
                for &i in &touched_h {
                    let c = hcount[i as usize];
                    hcount[i as usize] = 0;
                    if c == 2 {
                        setb(&mut halo, i as usize, true);
                    } else if c >= 3 {
                        setb(&mut halo, i as usize, false);
                    }
                }
                let mut reps = s.reps;
                let done: u32 = pat.iter().map(|w| w.count_ones()).sum();
                let strays: u32 = halo.iter().map(|w| w.count_ones()).sum();
                let mut cells = done;
                if done as usize == npat && strays == 0 {
                    reps += 1;
                    for w in pat.iter_mut() {
                        *w = 0;
                    }
                    cells = 0;
                }
                if reps as usize == count {
                    // reconstruct
                    let mut prog = vec![instr];
                    let mut p = pi;
                    for t in (1..=tick).rev() {
                        let st = &layers[t][p];
                        prog.push(st.instr);
                        p = st.parent as usize;
                    }
                    prog.reverse();
                    return Some(prog);
                }
                // potential: each emitter's ring distance to the nearest useful firing spot
                // (a missing pattern cell or a stray halo cell to erase)
                let mut pot = 0i64;
                let miss: &Vec<usize> = if cells == 0 && reps > s.reps { &(0..npat).collect::<Vec<_>>() } else { &missing };
                for k in 0..6 {
                    let mut best = n as i64;
                    for &c in miss {
                        if getb(&pat, c) {
                            continue;
                        }
                        let q = geo.where_[k][c];
                        if q == u16::MAX {
                            continue;
                        }
                        let d = (pos[k] as i64 - q as i64).abs();
                        let d = d.min(n as i64 - d);
                        if d < best {
                            best = d;
                        }
                    }
                    if strays > 0 {
                        for hw in 0..W {
                            let mut word = halo[hw];
                            while word != 0 {
                                let h = hw * 64 + word.trailing_zeros() as usize;
                                word &= word - 1;
                                let q = geo.hwhere[k][h];
                                if q == u16::MAX {
                                    continue;
                                }
                                let d = (pos[k] as i64 - q as i64).abs();
                                let d = d.min(n as i64 - d);
                                if d < best {
                                    best = d;
                                }
                            }
                        }
                    }
                    pot += best;
                }
                if let Some(ops) = &guide {
                    for op in ops {
                        let done = (0..W).all(|w| pat[w] & op.pat_mask[w] == op.pat_mask[w] && halo[w] & op.halo_mask[w] == 0);
                        if !done {
                            let mut gp = 0i64;
                            for &(e, q) in &op.targets {
                                gp += rdist(n, pos[e] as usize, q as usize);
                            }
                            pot = gp;
                            break;
                        }
                    }
                }
                let score = reps as i64 * 1_000_000 + cells as i64 * prm.cell_w - strays as i64 * prm.stray_pen - pot * prm.pot_w;
                let mut h = 0xcbf29ce484222325u64;
                for &x in pos.iter() {
                    h = (h ^ x as u64).wrapping_mul(0x100000001b3);
                }
                h = (h ^ on as u64).wrapping_mul(0x100000001b3);
                let mut h2 = reps as u64;
                for &w in pat.iter().chain(halo.iter()) {
                    h2 = (h2 ^ w).wrapping_mul(0x9E3779B97F4A7C15);
                }
                if !seen.insert((h, h2)) {
                    continue;
                }
                next.push(State { pos, on, pat, halo, reps, score, parent: pi as u32, instr });
                if next.len() >= 8 * width {
                    prune(&mut next, width, prm.cap);
                }
            }
        }
        prune(&mut next, width, prm.cap);
        layers.push(next);
    }
    None
}
