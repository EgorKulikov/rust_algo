use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const CAND: usize = 12;
const P: usize = 96; // AR order
const ST: usize = 4; // window stride for the AR fit
const WMAX: usize = 128;

// residual horizon past the boundary — longer for bigger chunks, so the
// window can reach past quiet gaps
fn horizon(cs: usize) -> usize {
    if let Ok(w) = std::env::var("B_W")
        && let Ok(w) = w.parse::<usize>()
    {
        return w.min(cs - 4);
    }
    (cs / 4).clamp(16, WMAX)
}

// Global AR(P) fit by least squares (covariance method): returns the
// whitening filter filt[0..=P] with filt[0] = 1, filt[k] = -coef[k].
fn fit_ar(a: &[f64], cs: usize) -> Vec<f64> {
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(8).min(16);
    // skip fit windows that cross corrupted-chunk boundaries — they contain
    // artificial glitches that pollute the model
    let ts: Vec<usize> = (P..a.len()).step_by(ST).filter(|&t| (t - P) / cs == t / cs).collect();
    let mut c = vec![0.0f64; P * P];
    let mut b = vec![0.0f64; P];
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in ts.chunks(ts.len().div_ceil(threads)) {
            handles.push(scope.spawn(move || {
                let mut lc = vec![0.0f64; P * P];
                let mut lb = vec![0.0f64; P];
                for &t in chunk {
                    let x = &a[t - P..t]; // x[P - 1 - k] is lag k+1
                    let y = a[t];
                    for i in 0..P {
                        let xi = x[P - 1 - i];
                        lb[i] += xi * y;
                        for j in i..P {
                            lc[i * P + j] += xi * x[P - 1 - j];
                        }
                    }
                }
                (lc, lb)
            }));
        }
        for h in handles {
            let (lc, lb) = h.join().unwrap();
            for k in 0..P * P {
                c[k] += lc[k];
            }
            for k in 0..P {
                b[k] += lb[k];
            }
        }
    });
    for i in 0..P {
        for j in 0..i {
            c[i * P + j] = c[j * P + i];
        }
    }
    let ridge = 1e-9 * (0..P).map(|i| c[i * P + i]).sum::<f64>() / P as f64;
    for i in 0..P {
        c[i * P + i] += ridge + 1e-6;
    }
    // Gaussian elimination with partial pivoting
    for col in 0..P {
        let piv = (col..P).max_by(|&x, &y| c[x * P + col].abs().total_cmp(&c[y * P + col].abs())).unwrap();
        if piv != col {
            for j in 0..P {
                c.swap(col * P + j, piv * P + j);
            }
            b.swap(col, piv);
        }
        let d = c[col * P + col];
        if d.abs() < 1e-30 {
            continue;
        }
        for row in col + 1..P {
            let f = c[row * P + col] / d;
            if f == 0.0 {
                continue;
            }
            for j in col..P {
                c[row * P + j] -= f * c[col * P + j];
            }
            b[row] -= f * b[col];
        }
    }
    let mut coef = vec![0.0f64; P];
    for col in (0..P).rev() {
        let mut v = b[col];
        for j in col + 1..P {
            v -= c[col * P + j] * coef[j];
        }
        let d = c[col * P + col];
        coef[col] = if d.abs() < 1e-30 { 0.0 } else { v / d };
    }
    let mut filt = vec![1.0f64];
    filt.extend(coef.iter().map(|&x| -x));
    filt
}

// Teacher-forced boundary residuals for one chunk under whitening filter:
// u[t] = contribution of this chunk's tail to the residual at position t past
// its end; v[t] = contribution of this chunk's head to the residual at
// position t past its predecessor's end.
fn uv_one(chunk: &[f64], filt: &[f64], w: usize) -> (Vec<f32>, Vec<f32>) {
    let cs = chunk.len();
    let p = P.min(cs);
    let tail = &chunk[cs - p..];
    let mut u = vec![0.0f32; w];
    let mut v = vec![0.0f32; w];
    for t in 0..w {
        let mut s = 0.0f64;
        for k in t + 1..=p {
            s += filt[k] * tail[p - (k - t)];
        }
        u[t] = s as f32;
        let mut s = 0.0f64;
        for k in 0..=t.min(P).min(cs - 1) {
            s += filt[k] * chunk[t - k];
        }
        v[t] = s as f32;
    }
    (u, v)
}

struct Feat {
    fdim: usize,
    fi: Vec<f32>, // successor-side: [u_fwd | v_bwd], n x fdim
    gj: Vec<f32>, // predecessor-side: [v_fwd | u_bwd], n x fdim
    fnorm: Vec<f32>,
    gnorm: Vec<f32>,
}

fn features(a: &[f64], n: usize, cs: usize) -> Feat {
    let arev: Vec<f64> = a.iter().rev().copied().collect();
    let filt_f = fit_ar(a, cs);
    let filt_b = fit_ar(&arev, cs);
    let w = horizon(cs);
    let fdim = 2 * w;
    let mut f = Feat {
        fdim,
        fi: Vec::with_capacity(n * fdim),
        gj: Vec::with_capacity(n * fdim),
        fnorm: Vec::with_capacity(n),
        gnorm: Vec::with_capacity(n),
    };
    for i in 0..n {
        let chunk = &a[i * cs..(i + 1) * cs];
        let zrev: Vec<f64> = chunk.iter().rev().copied().collect();
        let (uf, vf) = uv_one(chunk, &filt_f, w);
        let (ub, vb) = uv_one(&zrev, &filt_b, w);
        f.fnorm.push(uf.iter().chain(&vb).map(|x| x * x).sum());
        f.gnorm.push(vf.iter().chain(&ub).map(|x| x * x).sum());
        f.fi.extend_from_slice(&uf);
        f.fi.extend_from_slice(&vb);
        f.gj.extend_from_slice(&vf);
        f.gj.extend_from_slice(&ub);
    }
    f
}

// cost of chunk j directly following chunk i; direct sum-of-squares — the
// norm+dot rearrangement loses precision when residuals nearly cancel
fn cost(f: &Feat, i: usize, j: usize) -> f32 {
    let d = f.fdim;
    let (fi, gj) = (&f.fi[i * d..(i + 1) * d], &f.gj[j * d..(j + 1) * d]);
    let mut acc = 0.0f32;
    for k in 0..d {
        let s = fi[k] + gj[k];
        acc += s * s;
    }
    acc
}

const LRP: usize = 24; // AR order for the joint-fit likelihood-ratio test
const LRW: usize = 147; // samples per side in the LR window
const LRK: usize = 12; // candidates per tail to rescore with LR

fn solve_linear(c: &mut [f64], b: &mut [f64], p: usize) -> Vec<f64> {
    for col in 0..p {
        let piv = (col..p).max_by(|&x, &y| c[x * p + col].abs().total_cmp(&c[y * p + col].abs())).unwrap();
        if piv != col {
            for j in 0..p {
                c.swap(col * p + j, piv * p + j);
            }
            b.swap(col, piv);
        }
        let d = c[col * p + col];
        if d.abs() < 1e-30 {
            continue;
        }
        for row in col + 1..p {
            let f = c[row * p + col] / d;
            if f == 0.0 {
                continue;
            }
            for j in col..p {
                c[row * p + j] -= f * c[col * p + j];
            }
            b[row] -= f * b[col];
        }
    }
    let mut coef = vec![0.0f64; p];
    for col in (0..p).rev() {
        let mut v = b[col];
        for j in col + 1..p {
            v -= c[col * p + j] * coef[j];
        }
        let d = c[col * p + col];
        coef[col] = if d.abs() < 1e-30 { 0.0 } else { v / d };
    }
    coef
}

// mean squared one-step AR(LRP) residual of x under its own least-squares fit
fn ar_energy(x: &[f64]) -> f64 {
    let p = LRP;
    if x.len() <= p + 8 {
        return 0.0;
    }
    let mut c = vec![0.0f64; p * p];
    let mut b = vec![0.0f64; p];
    for t in p..x.len() {
        let y = x[t];
        for i in 0..p {
            let xi = x[t - 1 - i];
            b[i] += xi * y;
            for j in i..p {
                c[i * p + j] += xi * x[t - 1 - j];
            }
        }
    }
    for i in 0..p {
        for j in 0..i {
            c[i * p + j] = c[j * p + i];
        }
        c[i * p + i] += 1e-3;
    }
    let coef = solve_linear(&mut c, &mut b, p);
    let mut e = 0.0;
    for t in p..x.len() {
        let mut pred = 0.0;
        for i in 0..p {
            pred += coef[i] * x[t - 1 - i];
        }
        let d = x[t] - pred;
        e += d * d;
    }
    e / (x.len() - p) as f64
}

// Joint-fit likelihood ratio: does ONE model explain tail_i ++ head_j as well
// as separate models explain each half? ~1 for true joins, >1 for false.
fn lr_rescore(a: &[f64], n: usize, cs: usize, cand: &mut [Vec<(f32, u32)>]) {
    let l = LRW.min(cs);
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(8).min(16);
    let mut e_tail = vec![0.0f64; n];
    let mut e_head = vec![0.0f64; n];
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for t in 0..threads {
            let (et, eh): (Vec<(usize, f64)>, Vec<(usize, f64)>) = (Vec::new(), Vec::new());
            let mut et = et;
            let mut eh = eh;
            handles.push(scope.spawn(move || {
                let mut i = t;
                while i < n {
                    et.push((i, ar_energy(&a[(i + 1) * cs - l..(i + 1) * cs])));
                    eh.push((i, ar_energy(&a[i * cs..i * cs + l])));
                    i += threads;
                }
                (et, eh)
            }));
        }
        for h in handles {
            let (et, eh) = h.join().unwrap();
            for (i, e) in et {
                e_tail[i] = e;
            }
            for (i, e) in eh {
                e_head[i] = e;
            }
        }
    });
    let e_tail = &e_tail;
    let e_head = &e_head;
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for (t, chunk) in cand.chunks_mut(n.div_ceil(threads)).enumerate() {
            handles.push(scope.spawn(move || {
                let base = t * n.div_ceil(threads);
                for (off, list) in chunk.iter_mut().enumerate() {
                    let i = base + off;
                    let kr = LRK.min(list.len());
                    let mut joined = vec![0.0f64; 2 * l];
                    joined[..l].copy_from_slice(&a[(i + 1) * cs - l..(i + 1) * cs]);
                    for e in list[..kr].iter_mut() {
                        let j = e.1 as usize;
                        joined[l..].copy_from_slice(&a[j * cs..j * cs + l]);
                        let sep = 0.5 * (e_tail[i] + e_head[j]);
                        let lr = (ar_energy(&joined) / sep.max(1e-9)).max(1.0);
                        e.0 *= lr.powi(8) as f32;
                    }
                    list[..kr].sort_unstable_by(|x, y| x.0.total_cmp(&y.0));
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    });
}

const REFW: usize = 32; // samples per side for reference window matching
const CUT: usize = 14; // joins cut per repair round
const ROUNDS: usize = 4;

// Cut the worst-cost joins of the chain and re-splice the fragments optimally
// (Held-Karp over fragment ends). Fixes greedy commitment errors.
fn repair(perm: &mut Vec<usize>, feat: &Feat, pinned: &std::collections::HashSet<(usize, usize)>) {
    let n = perm.len();
    if n < 3 {
        return;
    }
    for _ in 0..ROUNDS {
        let mut joins: Vec<(f32, usize)> = (0..n - 1)
            .filter(|&t| !pinned.contains(&(perm[t], perm[t + 1])))
            .map(|t| (cost(feat, perm[t], perm[t + 1]), t))
            .collect();
        let b = CUT.min(joins.len());
        if b == 0 {
            return;
        }
        joins.select_nth_unstable_by(b - 1, |x, y| y.0.total_cmp(&x.0));
        let mut cuts: Vec<usize> = joins[..b].iter().map(|&(_, t)| t).collect();
        cuts.sort_unstable();
        let old_cost: f32 = joins[..b].iter().map(|&(c, _)| c).sum();
        // fragments as [start, end] index ranges of perm
        let mut frags = Vec::with_capacity(b + 1);
        let mut start = 0;
        for &c in &cuts {
            frags.push((start, c));
            start = c + 1;
        }
        frags.push((start, n - 1));
        let f = frags.len();
        let mut m = vec![f32::INFINITY; f * f];
        for x in 0..f {
            for y in 0..f {
                if x != y {
                    m[x * f + y] = cost(feat, perm[frags[x].1], perm[frags[y].0]);
                }
            }
        }
        // Held-Karp path over fragments
        let full = 1usize << f;
        let mut dp = vec![f32::INFINITY; full * f];
        let mut par = vec![u8::MAX; full * f];
        for x in 0..f {
            dp[(1 << x) * f + x] = 0.0;
        }
        for mask in 1..full {
            for last in 0..f {
                let d = dp[mask * f + last];
                if !d.is_finite() || mask & (1 << last) == 0 {
                    continue;
                }
                for nxt in 0..f {
                    if mask & (1 << nxt) != 0 {
                        continue;
                    }
                    let nd = d + m[last * f + nxt];
                    let idx = (mask | (1 << nxt)) * f + nxt;
                    if nd < dp[idx] {
                        dp[idx] = nd;
                        par[idx] = last as u8;
                    }
                }
            }
        }
        let last = (0..f).min_by(|&x, &y| {
            dp[(full - 1) * f + x].total_cmp(&dp[(full - 1) * f + y])
        }).unwrap();
        if dp[(full - 1) * f + last] + 1e-3 >= old_cost {
            return; // no improvement possible
        }
        let mut order = vec![last];
        let mut mask = full - 1;
        let mut cur = last;
        while par[mask * f + cur] != u8::MAX {
            let p = par[mask * f + cur] as usize;
            mask ^= 1 << cur;
            cur = p;
            order.push(cur);
        }
        order.reverse();
        if order.iter().enumerate().all(|(k, &x)| k == x) {
            return; // already optimal
        }
        let mut np = Vec::with_capacity(n);
        for &x in &order {
            np.extend_from_slice(&perm[frags[x].0..=frags[x].1]);
        }
        *perm = np;
    }
}

fn whash(w: &[i32]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &x in w {
        h = h.wrapping_mul(0x100000001b3).wrapping_add(x as u32 as u64);
    }
    h
}

// If a previous record of the same stage was solved, its restored audio can
// pin down successors exactly: the 2*REFW-sample window around a true join
// appears verbatim in the reference wherever it was reconstructed correctly.
fn ref_hints(a: &[f64], n: usize, cs: usize) -> Vec<(usize, usize)> {
    let Ok(paths) = std::env::var("B_REF") else { return Vec::new() };
    let mut cands: Vec<std::collections::HashSet<usize>> = vec![Default::default(); n];
    for path in paths.split(':') {
        let Ok(text) = std::fs::read_to_string(path) else { continue };
        for (i, j) in ref_hints_one(&text, a, n, cs) {
            cands[i].insert(j);
        }
    }
    (0..n)
        .filter(|&i| cands[i].len() == 1)
        .map(|i| (i, *cands[i].iter().next().unwrap()))
        .collect()
}

fn ref_hints_one(text: &str, a: &[f64], n: usize, cs: usize) -> Vec<(usize, usize)> {
    let mut it = text.split_ascii_whitespace().map(|t| t.parse::<i32>().unwrap_or(0));
    let rlen = it.next().unwrap_or(0) as usize;
    let r: Vec<i32> = it.take(rlen).collect();
    if r.len() < 2 * REFW || cs < REFW {
        return Vec::new();
    }
    let ai = |idx: usize| a[idx] as i32;
    // positions of every REFW-window in the reference, sorted by hash
    let mut pos: Vec<(u64, u32)> =
        (0..=r.len() - 2 * REFW).map(|p| (whash(&r[p..p + REFW]), p as u32)).collect();
    pos.sort_unstable();
    // chunk heads by hash
    let mut heads: std::collections::HashMap<u64, Vec<u32>> = std::collections::HashMap::new();
    for j in 0..n {
        let head: Vec<i32> = (0..REFW).map(|k| ai(j * cs + k)).collect();
        heads.entry(whash(&head)).or_default().push(j as u32);
    }
    let mut hints = Vec::new();
    for i in 0..n {
        let tail: Vec<i32> = (0..REFW).map(|k| ai((i + 1) * cs - REFW + k)).collect();
        let h = whash(&tail);
        let lo = pos.partition_point(|&(x, _)| x < h);
        let mut cand: Option<usize> = None;
        let mut ok = true;
        for &(_, p) in pos[lo..].iter().take_while(|&&(x, _)| x == h) {
            let p = p as usize;
            if r[p..p + REFW] != tail[..] {
                continue; // hash collision
            }
            let cont = &r[p + REFW..p + 2 * REFW];
            let Some(js) = heads.get(&whash(cont)) else { continue };
            for &j in js {
                let j = j as usize;
                if j == i || (0..REFW).any(|k| ai(j * cs + k) != cont[k]) {
                    continue;
                }
                match cand {
                    None => cand = Some(j),
                    Some(c) if c != j => ok = false,
                    _ => {}
                }
            }
        }
        if ok && let Some(j) = cand {
            hints.push((i, j));
        }
    }
    hints
}

fn find(comp: &mut Vec<usize>, mut x: usize) -> usize {
    while comp[x] != x {
        comp[x] = comp[comp[x]];
        x = comp[x];
    }
    x
}

// Max-regret chain building: repeatedly commit the tail with the LARGEST gap
// between its best and second-best still-available successor. Ambiguous
// (flat-profile) tails are decided last, when elimination constrains them.
fn build_chains_regret(
    n: usize,
    cand: &[Vec<(f32, u32)>], // per-tail candidates sorted by cost asc
    forced: &[(usize, usize)],
) -> (Vec<usize>, Vec<usize>) {
    let mut next = vec![usize::MAX; n];
    let mut prev = vec![usize::MAX; n];
    let mut comp: Vec<usize> = (0..n).collect();
    for &(i, j) in forced {
        if next[i] == usize::MAX && prev[j] == usize::MAX && find(&mut comp, i) != find(&mut comp, j)
        {
            let (ci, cj) = (find(&mut comp, i), find(&mut comp, j));
            comp[ci] = cj;
            next[i] = j;
            prev[j] = i;
        }
    }
    // (best cost, best j, cost of runner-up) among still-valid candidates
    fn valid2(
        i: usize,
        cand: &[Vec<(f32, u32)>],
        prev: &[usize],
        comp: &mut Vec<usize>,
    ) -> Option<(f32, u32, f32)> {
        let ci = find(comp, i);
        let mut first: Option<(f32, u32)> = None;
        for &(c, j) in &cand[i] {
            if prev[j as usize] != usize::MAX || find(comp, j as usize) == ci {
                continue;
            }
            if let Some((fc, fj)) = first {
                return Some((fc, fj, c));
            }
            first = Some((c, j));
        }
        first.map(|(fc, fj)| (fc, fj, f32::MAX / 4.0))
    }
    let mut heap: std::collections::BinaryHeap<(i64, u32, u32)> = std::collections::BinaryHeap::new();
    for i in 0..n {
        if next[i] != usize::MAX {
            continue;
        }
        if let Some((c1, j, c2)) = valid2(i, cand, &prev, &mut comp) {
            heap.push(((((c2 - c1) as f64) * 16.0) as i64, i as u32, j));
        }
    }
    while let Some((r, i, j)) = heap.pop() {
        let i = i as usize;
        if next[i] != usize::MAX {
            continue;
        }
        if let Some((c1, j2, c2)) = valid2(i, cand, &prev, &mut comp) {
            let cur = (((c2 - c1) as f64) * 16.0) as i64;
            if cur < r || j2 != j {
                heap.push((cur, i as u32, j2)); // stale, requeue with fresh regret
                continue;
            }
            let jj = j2 as usize;
            let (ci, cj) = (find(&mut comp, i), find(&mut comp, jj));
            comp[ci] = cj;
            next[i] = jj;
            prev[jj] = i;
        }
    }
    (next, prev)
}

fn build_chains(n: usize, mut edges: Vec<(i64, u32, u32)>) -> (Vec<usize>, Vec<usize>) {
    edges.sort_unstable();
    let mut next = vec![usize::MAX; n];
    let mut prev = vec![usize::MAX; n];
    let mut comp: Vec<usize> = (0..n).collect();
    fn find(comp: &mut Vec<usize>, mut x: usize) -> usize {
        while comp[x] != x {
            comp[x] = comp[comp[x]];
            x = comp[x];
        }
        x
    }
    for (_, i, j) in edges {
        let (i, j) = (i as usize, j as usize);
        if next[i] != usize::MAX || prev[j] != usize::MAX {
            continue;
        }
        let (ci, cj) = (find(&mut comp, i), find(&mut comp, j));
        if ci == cj {
            continue;
        }
        comp[ci] = cj;
        next[i] = j;
        prev[j] = i;
    }
    (next, prev)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let size: usize = input.read();
    let a: Vec<i32> = input.read_vec(size);
    let a: Vec<f64> = a.into_iter().map(|x| x as f64).collect();
    let cs = size / n;
    let feat = features(&a, n, cs);

    // Top-CAND candidate successors for every chunk, in parallel.
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(8).min(16);
    let mut cand: Vec<Vec<(f32, u32)>> = vec![Vec::new(); n];
    std::thread::scope(|scope| {
        let feat = &feat;
        let mut handles = Vec::new();
        for t in 0..threads {
            handles.push(scope.spawn(move || {
                let mut local = Vec::new();
                let mut i = t;
                while i < n {
                    let mut best: Vec<(f32, u32)> = Vec::with_capacity(CAND + 1);
                    for j in 0..n {
                        if i == j {
                            continue;
                        }
                        let c = cost(feat, i, j);
                        if best.len() < CAND || c < best.last().unwrap().0 {
                            let pos = best.partition_point(|&(bc, _)| bc <= c);
                            best.insert(pos, (c, j as u32));
                            if best.len() > CAND {
                                best.pop();
                            }
                        }
                    }
                    local.push((i, best));
                    i += threads;
                }
                local
            }));
        }
        for h in handles {
            for (i, best) in h.join().unwrap() {
                cand[i] = best;
            }
        }
    });
    if std::env::var("B_LR").as_deref() != Ok("off") {
        lr_rescore(&a, n, cs, &mut cand);
    }
    // reference-derived successors are near-certain: force them
    let hints = ref_hints(&a, n, cs);
    let (next, prev) = if std::env::var("B_CHAIN").as_deref() == Ok("regret") {
        build_chains_regret(n, &cand, &hints)
    } else {
        let mut edges: Vec<(i64, u32, u32)> = cand
            .iter()
            .enumerate()
            .flat_map(|(i, cs)| cs.iter().map(move |&(c, j)| ((c * 16.0) as i64, i as u32, j)))
            .collect();
        for (t, &(i, j)) in hints.iter().enumerate() {
            edges.push((i64::MIN + t as i64, i as u32, j as u32));
        }
        build_chains(n, edges)
    };

    // Collect chain fragments and join them with a second Kruskal pass over
    // all tail->head pairs between fragments.
    let mut frags = Vec::new();
    for i in 0..n {
        if prev[i] == usize::MAX {
            let mut frag = vec![i];
            let mut cur = i;
            while next[cur] != usize::MAX {
                cur = next[cur];
                frag.push(cur);
            }
            frags.push(frag);
        }
    }
    let f = frags.len();
    let mut edges = Vec::with_capacity(f * f);
    for x in 0..f {
        for y in 0..f {
            if x != y {
                let c = cost(&feat, *frags[x].last().unwrap(), frags[y][0]);
                edges.push(((c * 16.0) as i64, x as u32, y as u32));
            }
        }
    }
    let (fnext, fprev) = build_chains(f, edges);
    let start = (0..f).find(|&x| fprev[x] == usize::MAX).unwrap();
    let mut perm = Vec::with_capacity(n);
    let mut cur = start;
    loop {
        perm.extend_from_slice(&frags[cur]);
        if fnext[cur] == usize::MAX {
            break;
        }
        cur = fnext[cur];
    }
    assert_eq!(perm.len(), n);
    // NB: cut-and-splice repair (fn repair) measured NEGATIVE on lccs — cheaper
    // cost configurations exist that break true runs. Disabled.
    let _ = &hints;
    out.print_line_iter(perm.into_iter());
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}


#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
