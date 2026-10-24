//! Level 6, exact search for small boards (radius <= 4). A repetition is
//! searched by iterative deepening over ticks (all 4^6 instruction combos,
//! incremental admissible lower bound, transposition table); repetitions are
//! chained by a layered search that keeps the best K post-submission states
//! per layer.
//!
//! State = ring index of each emitter, on/off bits, bitmap over the pattern
//! cells and their halo (anything else never matters). After a submission
//! the bitmap is always empty.

use crate::level4::Machine;
use crate::level5::{neighbours, Pattern};
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::time::{Duration, Instant};

const NONE: u8 = u8::MAX;
const DMAX: usize = 7;
const INSTR: [u8; 4] = [b'-', b'F', b'B', b'T'];

#[derive(Default)]
struct IdHasher(u64);
impl Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ b as u64).wrapping_mul(0x100000001b3);
        }
    }
    fn write_u128(&mut self, v: u128) {
        let lo = v as u64;
        let hi = (v >> 64) as u64;
        let mut h = lo.wrapping_mul(0x9E3779B97F4A7C15) ^ hi.wrapping_mul(0xC2B2AE3D27D4EB4F);
        h ^= h >> 29;
        h = h.wrapping_mul(0xBF58476D1CE4E5B9);
        h ^= h >> 32;
        self.0 = h;
    }
}
type Map<V> = HashMap<u128, V, BuildHasherDefault<IdHasher>>;

#[derive(Clone, Copy, PartialEq, Eq)]
struct St {
    pos: [u8; 6],
    on: u8,
    bm: u64,
    /// per emitter, while off: 0 free, 1 last stayed, 2 last moved F, 3 last moved B
    run: [u8; 6],
}

impl St {
    fn key(&self) -> u128 {
        let mut k = 0u64;
        for &p in &self.pos {
            k = k << 8 | p as u64;
        }
        k = k << 8 | self.on as u64;
        (k as u128) << 64 | self.bm as u128
    }
}

struct Geo {
    n: usize,
    npat: usize,
    pat_mask: u64,
    /// beam[k * n + p]: cells crossed by emitter k firing from ring index p.
    beam: Vec<u64>,
    /// where_[k * ncell + c]: ring index from which emitter k crosses cell c.
    where_: Vec<u8>,
    ncell: usize,
}

impl Geo {
    fn ringdist(&self, a: u8, b: u8) -> u32 {
        let d = (a as i32 - b as i32).unsigned_abs();
        d.min(self.n as u32 - d)
    }

    /// Ring index and on-state of emitter k after instruction `a` from (p, on).
    fn step(&self, p: usize, was_on: bool, a: usize) -> (usize, bool) {
        match a {
            0 => (p, was_on),
            1 => ((p + 1) % self.n, was_on),
            2 => ((p + self.n - 1) % self.n, was_on),
            _ => (p, !was_on),
        }
    }

    /// Admissible lower bound on the ticks needed to reach a submission
    /// (used only to rank post-submission states).
    fn h(&self, st: &St) -> u32 {
        let missing = !st.bm & self.pat_mask;
        let strays = st.bm & !self.pat_mask;
        if missing == 0 && strays == 0 {
            return 0;
        }
        let mut need = [[NONE; 64]; 6];
        let active = missing | strays;
        for k in 0..6 {
            let extra = if st.on >> k & 1 == 1 { 0 } else { 1 };
            let mut m = active;
            while m != 0 {
                let c = m.trailing_zeros() as usize;
                m &= m - 1;
                let w = self.where_[k * self.ncell + c];
                if w != NONE {
                    need[k][c] = (self.ringdist(st.pos[k], w) + extra) as u8;
                }
            }
        }
        let mut hmax = 1u32;
        let mut m = missing;
        while m != 0 {
            let c = m.trailing_zeros() as usize;
            m &= m - 1;
            let mut best = NONE;
            for a in 0..6 {
                let na = need[a][c];
                if na >= best {
                    continue;
                }
                for b in a + 1..6 {
                    let v = na.max(need[b][c]);
                    if v < best {
                        best = v;
                    }
                }
            }
            hmax = hmax.max(best as u32);
        }
        let mut m = strays;
        while m != 0 {
            let c = m.trailing_zeros() as usize;
            m &= m - 1;
            let mut v = [
                need[0][c], need[1][c], need[2][c], need[3][c], need[4][c], need[5][c],
            ];
            v.sort_unstable();
            hmax = hmax.max(v[2] as u32);
        }
        hmax
    }
}

struct Res {
    st: St,
    prog: Vec<[u8; 6]>,
}

/// Bit-sliced counters: cells covered by >= 1, >= 2, >= 3 of the items added so far.
#[derive(Clone, Copy)]
struct Cnt(u64, u64, u64);
impl Cnt {
    fn add(self, m: u64) -> Cnt {
        Cnt(self.0 | m, self.1 | (self.0 & m), self.2 | (self.1 & m))
    }
}

struct Searcher<'a> {
    geo: &'a Geo,
    /// state -> largest remaining-tick bound proven infeasible
    fail: Map<u8>,
    /// per search: state -> largest bound already explored
    vis: Map<u8>,
    results: Vec<Res>,
    path: [[u8; 6]; DMAX + 1],
    final_mode: bool,
    stop: bool,
    aborted: bool,
    leaves: u64,
    deadline: Instant,
    max_results: usize,
    /// prune ticks that destroy an already built pattern cell (heuristic, default on)
    strict: bool,
    /// per depth: suffix counts over emitters k..6 of the cells they could cover
    suf: [[Cnt; 7]; DMAX + 1],
    /// per depth: cover[k][a] = cells whose need stays within the bound if emitter k takes option a
    cover: [[[u64; 4]; 6]; DMAX + 1],
}

impl<'a> Searcher<'a> {
    fn check_time(&mut self) {
        self.leaves += 1;
        if self.leaves & 0xffff == 0 && Instant::now() > self.deadline {
            self.aborted = true;
            self.stop = true;
        }
    }

    /// Shortest repetition from `start`: the depth; end states are left in `results`.
    /// On abort the results found so far at the current depth (if any) are kept.
    fn search(&mut self, start: St, final_mode: bool) -> Option<usize> {
        self.final_mode = final_mode;
        self.stop = false;
        self.aborted = false;
        for d in 1..=DMAX {
            self.vis.clear();
            self.results.clear();
            self.dfs(start, d, 0);
            if !self.results.is_empty() {
                return Some(d);
            }
            if self.aborted {
                return None;
            }
            if self.fail.len() > 6_000_000 {
                self.fail.clear();
            }
        }
        None
    }

    fn fill_suffix(&mut self, depth: usize, reach: &[u64; 6]) {
        let mut c = Cnt(0, 0, 0);
        self.suf[depth][6] = c;
        for k in (0..6).rev() {
            c = c.add(reach[k]);
            self.suf[depth][k] = c;
        }
    }

    fn dfs(&mut self, st: St, r: usize, depth: usize) {
        let geo = self.geo;
        let missing = !st.bm & geo.pat_mask;
        let strays = st.bm & !geo.pat_mask;
        if r == 1 {
            self.last_tick(st, missing, strays, depth);
            return;
        }
        // covers for the bound r - 1 on the child
        let bound = (r - 1) as u32;
        let active = missing | strays;
        let mut reach = [0u64; 6];
        for k in 0..6 {
            let p = st.pos[k] as usize;
            let was_on = st.on >> k & 1 == 1;
            for a in 0..4 {
                let (np, non) = geo.step(p, was_on, a);
                let extra = if non { 0 } else { 1 };
                let mut cov = 0u64;
                let mut m = active;
                while m != 0 {
                    let c = m.trailing_zeros() as usize;
                    m &= m - 1;
                    let w = geo.where_[k * geo.ncell + c];
                    if w != NONE && geo.ringdist(np as u8, w) + extra <= bound {
                        cov |= 1 << c;
                    }
                }
                self.cover[depth][k][a] = cov;
                reach[k] |= cov;
            }
        }
        self.fill_suffix(depth, &reach);
        self.expand(
            &st,
            0,
            st.pos,
            st.on,
            st.run,
            Cnt(0, 0, 0),
            [b'-'; 6],
            Cnt(0, 0, 0),
            missing,
            strays,
            r,
            depth,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn expand(
        &mut self,
        st: &St,
        k: usize,
        pos: [u8; 6],
        on: u8,
        run: [u8; 6],
        beams: Cnt,
        instr: [u8; 6],
        cov: Cnt,
        missing: u64,
        strays: u64,
        r: usize,
        depth: usize,
    ) {
        if self.stop {
            return;
        }
        let s = self.suf[depth][k];
        // every missing cell needs two emitters within the bound, every stray three
        if missing & !(cov.1 | (cov.0 & s.0) | s.1) != 0 {
            return;
        }
        if strays & !(cov.2 | (cov.1 & s.0) | (cov.0 & s.1) | s.2) != 0 {
            return;
        }
        if k == 6 {
            self.check_time();
            let geo = self.geo;
            if self.strict && beams.2 & st.bm & geo.pat_mask != 0 {
                return;
            }
            let nbm = (st.bm | (beams.1 & !beams.2)) & !beams.2;
            if nbm == geo.pat_mask {
                // submitted earlier than the bound
                self.path[depth] = instr;
                self.record(
                    St {
                        pos,
                        on,
                        bm: 0,
                        run,
                    },
                    depth + 1,
                );
                return;
            }
            let child = St {
                pos,
                on,
                bm: nbm,
                run,
            };
            let need = (r - 1) as u8;
            let key = child.key();
            if let Some(&f) = self.fail.get(&key) {
                if f >= need {
                    return;
                }
            }
            match self.vis.get_mut(&key) {
                Some(v) if *v >= need => return,
                Some(v) => *v = need,
                None => {
                    self.vis.insert(key, need);
                }
            }
            self.path[depth] = instr;
            let before = self.results.len();
            self.dfs(child, r - 1, depth + 1);
            if self.results.len() == before && !self.aborted {
                let e = self.fail.entry(key).or_insert(0);
                if *e < need {
                    *e = need;
                }
            }
            return;
        }
        let geo = self.geo;
        let p = pos[k] as usize;
        let was_on = on >> k & 1 == 1;
        let m = st.run[k];
        for a in 0..4 {
            // off emitters: moves before stays, one direction only (canonical order)
            if !was_on {
                let ok = match a {
                    1 => m == 0 || m == 2,
                    2 => m == 0 || m == 3,
                    _ => true,
                };
                if !ok {
                    continue;
                }
            }
            let (np, non) = geo.step(p, was_on, a);
            let mut run2 = run;
            run2[k] = if non || a == 3 { 0 } else { a as u8 + 1 };
            let mut pos2 = pos;
            pos2[k] = np as u8;
            let on2 = if non { on | 1 << k } else { on & !(1 << k) };
            let mut instr2 = instr;
            instr2[k] = INSTR[a];
            let beams2 = if non {
                beams.add(geo.beam[k * geo.n + np])
            } else {
                beams
            };
            let cov2 = cov.add(self.cover[depth][k][a]);
            self.expand(
                st,
                k + 1,
                pos2,
                on2,
                run2,
                beams2,
                instr2,
                cov2,
                missing,
                strays,
                r,
                depth,
            );
        }
    }

    fn record(&mut self, st: St, len: usize) {
        if self.results.len() < self.max_results {
            self.results.push(Res {
                st,
                prog: self.path[..len].to_vec(),
            });
        }
        if self.final_mode || self.results.len() >= self.max_results {
            self.stop = true;
        }
    }

    fn last_tick(&mut self, st: St, missing: u64, strays: u64, depth: usize) {
        let geo = self.geo;
        let n = geo.n;
        // reach[k]: cells emitter k can cross this tick
        let mut reach = [0u64; 6];
        for k in 0..6 {
            let p = st.pos[k] as usize;
            reach[k] = geo.beam[k * n + p];
            if st.on >> k & 1 == 1 {
                reach[k] |= geo.beam[k * n + (p + 1) % n] | geo.beam[k * n + (p + n - 1) % n];
            }
        }
        self.fill_suffix(depth, &reach);
        self.last_expand(
            &st,
            0,
            st.pos,
            st.on,
            Cnt(0, 0, 0),
            [b'-'; 6],
            0,
            missing,
            strays,
            depth,
        );
    }

    /// Last tick of a repetition: every missing cell must get exactly two beams, every
    /// stray at least three, no pattern cell three. `var` marks emitters that are off
    /// and stay off (their move is irrelevant for the board; expanded on success).
    #[allow(clippy::too_many_arguments)]
    fn last_expand(
        &mut self,
        st: &St,
        k: usize,
        pos: [u8; 6],
        on: u8,
        beams: Cnt,
        instr: [u8; 6],
        var: u8,
        missing: u64,
        strays: u64,
        depth: usize,
    ) {
        if self.stop {
            return;
        }
        let geo = self.geo;
        if beams.2 & geo.pat_mask != 0 {
            return;
        }
        let s = self.suf[depth][k];
        if missing & !(beams.1 | (beams.0 & s.0) | s.1) != 0 {
            return;
        }
        if strays & !(beams.2 | (beams.1 & s.0) | (beams.0 & s.1) | s.2) != 0 {
            return;
        }
        if k == 6 {
            self.check_time();
            let nbm = (st.bm | (beams.1 & !beams.2)) & !beams.2;
            if nbm != geo.pat_mask {
                return;
            }
            // success: expand the free moves of emitters that stayed off
            let n = geo.n;
            let mut vars = [0usize; 6];
            let mut nv = 0;
            for e in 0..6 {
                if var >> e & 1 == 1 {
                    vars[nv] = e;
                    nv += 1;
                }
            }
            let combos = if self.final_mode {
                1
            } else {
                3usize.pow(nv as u32)
            };
            for c in 0..combos {
                let mut pos2 = pos;
                let mut instr2 = instr;
                let mut cc = c;
                let mut canonical = true;
                let mut run2 = [0u8; 6];
                for &e in &vars[..nv] {
                    let a = cc % 3;
                    cc /= 3;
                    run2[e] = a as u8 + 1;
                    let m = st.run[e];
                    canonical &= match a {
                        1 => m == 0 || m == 2,
                        2 => m == 0 || m == 3,
                        _ => true,
                    };
                    pos2[e] = geo.step(st.pos[e] as usize, false, a).0 as u8;
                    instr2[e] = INSTR[a];
                }
                if !canonical {
                    continue;
                }
                self.path[depth] = instr2;
                self.record(
                    St {
                        pos: pos2,
                        on,
                        bm: 0,
                        run: run2,
                    },
                    depth + 1,
                );
                if self.stop {
                    return;
                }
            }
            return;
        }
        let n = geo.n;
        let p = pos[k] as usize;
        let was_on = on >> k & 1 == 1;
        if was_on {
            for a in 0..4 {
                let (np, non) = geo.step(p, was_on, a);
                let mut pos2 = pos;
                pos2[k] = np as u8;
                let mut instr2 = instr;
                instr2[k] = INSTR[a];
                if non {
                    self.last_expand(
                        st,
                        k + 1,
                        pos2,
                        on,
                        beams.add(geo.beam[k * n + np]),
                        instr2,
                        var,
                        missing,
                        strays,
                        depth,
                    );
                } else {
                    self.last_expand(
                        st,
                        k + 1,
                        pos2,
                        on & !(1 << k),
                        beams,
                        instr2,
                        var,
                        missing,
                        strays,
                        depth,
                    );
                }
            }
        } else {
            // toggle on in place
            let mut instr2 = instr;
            instr2[k] = b'T';
            self.last_expand(
                st,
                k + 1,
                pos,
                on | 1 << k,
                beams.add(geo.beam[k * n + p]),
                instr2,
                var,
                missing,
                strays,
                depth,
            );
            // stay off (move expanded on success)
            self.last_expand(
                st,
                k + 1,
                pos,
                on,
                beams,
                instr,
                var | 1 << k,
                missing,
                strays,
                depth,
            );
        }
    }
}

fn build_geo(radius: usize, pattern: &Pattern) -> Option<(Geo, St)> {
    let m = Machine::new(radius);
    let n = m.ring.len();
    let mut cells: Vec<(i32, i32)> = pattern.cells.clone();
    let npat = cells.len();
    for &c in &pattern.cells {
        for nb in neighbours(c) {
            if m.board.get(nb.0, nb.1) == b'O' && !cells.contains(&nb) {
                cells.push(nb);
            }
        }
    }
    let ncell = cells.len();
    if ncell > 64 || n > 250 {
        return None;
    }
    let mut beam = vec![0u64; 6 * n];
    let mut where_ = vec![NONE; 6 * ncell];
    for k in 0..6 {
        let (dr, dc) = crate::board::direction(k as u8 + 1);
        for p in 0..n {
            let (mut r, mut c) = m.ring[p];
            r += dr;
            c += dc;
            while matches!(m.board.get(r, c), b'O' | b'M') {
                if let Some(i) = cells.iter().position(|&x| x == (r, c)) {
                    beam[k * n + p] |= 1 << i;
                    where_[k * ncell + i] = p as u8;
                }
                r += dr;
                c += dc;
            }
        }
    }
    let pat_mask = if npat == 64 {
        u64::MAX
    } else {
        (1u64 << npat) - 1
    };
    let geo = Geo {
        n,
        npat,
        pat_mask,
        beam,
        where_,
        ncell,
    };
    let st = St {
        pos: [
            m.pos[0] as u8,
            m.pos[1] as u8,
            m.pos[2] as u8,
            m.pos[3] as u8,
            m.pos[4] as u8,
            m.pos[5] as u8,
        ],
        on: 0,
        bm: 0,
        run: [0; 6],
    };
    Some((geo, st))
}

struct Node {
    st: St,
    ticks: usize,
    prog: Vec<[u8; 6]>,
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

pub fn exact_build(
    radius: usize,
    pattern: &Pattern,
    count: usize,
    time_budget_ms: u64,
) -> Option<Vec<[u8; 6]>> {
    let (geo, start) = build_geo(radius, pattern)?;
    if geo.npat == 0 || count == 0 {
        return None;
    }
    let t0 = Instant::now();
    let budget = Duration::from_millis(time_budget_ms);
    let hard = t0 + budget * 2 + Duration::from_millis(500);
    let k_keep = env_usize("NANO_EXACT_K", 64);
    let max_results = env_usize("NANO_EXACT_RES", 2048);
    let strict = env_usize("NANO_EXACT_STRICT", 1) != 0;
    let dbg = std::env::var("NANO_EXACT_DBG").is_ok();
    let mut s = Searcher {
        geo: &geo,
        fail: Map::default(),
        vis: Map::default(),
        results: Vec::new(),
        path: [[b'-'; 6]; DMAX + 1],
        final_mode: false,
        stop: false,
        aborted: false,
        leaves: 0,
        deadline: hard,
        max_results,
        strict,
        suf: [[Cnt(0, 0, 0); 7]; DMAX + 1],
        cover: [[[0; 4]; 6]; DMAX + 1],
    };
    let mut layer: Vec<Node> = vec![Node {
        st: start,
        ticks: 0,
        prog: Vec::new(),
    }];
    for rep in 0..count {
        let final_mode = rep + 1 == count;
        let now = Instant::now();
        // share what is left of the budget evenly over the remaining repetitions
        let left = (t0 + budget).saturating_duration_since(now);
        let layer_end = now + left / (count - rep) as u32;
        let mut next: Map<Node> = Map::default();
        let mut best: Option<Node> = None;
        let mut expanded = 0usize;
        for (si, node) in layer.iter().take(k_keep).enumerate() {
            if let Some(b) = &best {
                if node.ticks + 1 >= b.ticks {
                    break;
                }
            }
            if si > 0 && Instant::now() > layer_end {
                break;
            }
            // the first start of a layer may use all the time there is
            s.deadline = if si == 0 { hard } else { layer_end };
            expanded += 1;
            let found = s.search(node.st, final_mode);
            if found.is_none() {
                if s.aborted {
                    break;
                }
                continue;
            }
            for res in s.results.drain(..) {
                let ticks = node.ticks + res.prog.len();
                let mut prog = node.prog.clone();
                prog.extend_from_slice(&res.prog);
                if final_mode {
                    if best.as_ref().map_or(true, |b| ticks < b.ticks) {
                        best = Some(Node {
                            st: res.st,
                            ticks,
                            prog,
                        });
                    }
                    break;
                }
                let key = res.st.key();
                match next.get_mut(&key) {
                    Some(e) if e.ticks <= ticks => {}
                    Some(e) => {
                        e.ticks = ticks;
                        e.prog = prog;
                    }
                    None => {
                        next.insert(
                            key,
                            Node {
                                st: res.st,
                                ticks,
                                prog,
                            },
                        );
                    }
                }
            }
            if s.aborted {
                break;
            }
        }
        if dbg {
            eprintln!(
                "  rep {} expanded {}/{} next {} ticks {:?} leaves {} t={:?}",
                rep,
                expanded,
                layer.len(),
                next.len(),
                next.values()
                    .map(|n| n.ticks)
                    .min()
                    .or(best.as_ref().map(|b| b.ticks)),
                s.leaves,
                t0.elapsed()
            );
        }
        if final_mode {
            return best.map(|b| b.prog);
        }
        let mut v: Vec<(usize, u32, u64, Node)> = next
            .into_values()
            .map(|nd| {
                let h = geo.h(&nd.st);
                let tie = (nd.st.key() as u64).wrapping_mul(0x9E3779B97F4A7C15) >> 40;
                (nd.ticks, h, tie, nd)
            })
            .collect();
        if v.is_empty() {
            return None;
        }
        v.sort_unstable_by(|a, b| (a.0, a.1, a.2).cmp(&(b.0, b.1, b.2)));
        layer = v.into_iter().map(|x| x.3).collect();
    }
    None
}
