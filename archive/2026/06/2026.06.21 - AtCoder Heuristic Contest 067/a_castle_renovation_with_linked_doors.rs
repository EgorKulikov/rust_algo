
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::io::output::BoolOutput;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Write;
use std::time::Instant;

type PreCalc = ();

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const TREE_TRIES: usize = 300;

// Canonical door descriptor for the edge between two adjacent cells.
// dir 0 = between (i,j) and (i+1,j); dir 1 = between (i,j) and (i,j+1).
fn edge(a: (usize, usize), b: (usize, usize)) -> (u8, usize, usize) {
    if a.0 + 1 == b.0 && a.1 == b.1 {
        (0, a.0, a.1)
    } else if b.0 + 1 == a.0 && a.1 == b.1 {
        (0, b.0, b.1)
    } else if a.1 + 1 == b.1 && a.0 == b.0 {
        (1, a.0, a.1)
    } else {
        (1, b.0, b.1)
    }
}

// Tree distance (BFS) from `src` over the tree adjacency, skipping cut cells.
fn tree_bfs(
    adj: &[Vec<Vec<(usize, usize)>>],
    cut: &[Vec<bool>],
    n: usize,
    src: (usize, usize),
) -> Vec<Vec<i32>> {
    let mut dist = vec![vec![-1i32; n]; n];
    if cut[src.0][src.1] {
        return dist;
    }
    let mut q = VecDeque::new();
    dist[src.0][src.1] = 0;
    q.push_back(src);
    while let Some((i, j)) = q.pop_front() {
        for &(ni, nj) in &adj[i][j] {
            if !cut[ni][nj] && dist[ni][nj] == -1 {
                dist[ni][nj] = dist[i][j] + 1;
                q.push_back((ni, nj));
            }
        }
    }
    dist
}

// Random-DFS spanning tree rooted at (0,0); the throne is never expanded, so
// it stays a leaf. Random neighbour order gives a different tree each call.
fn random_dfs(
    grid: &[Vec<u8>],
    n: usize,
    throne: (usize, usize),
    rng: &mut impl RandomTrait,
) -> (Vec<Vec<bool>>, Vec<Vec<(usize, usize)>>) {
    let mut vis = vec![vec![false; n]; n];
    let mut parent = vec![vec![(usize::MAX, usize::MAX); n]; n];
    vis[0][0] = true;
    let mut stack = vec![(0usize, 0usize)];
    let mut buf: Vec<(usize, usize)> = Vec::new();
    while let Some(&(i, j)) = stack.last() {
        if (i, j) == throne {
            stack.pop();
            continue;
        }
        buf.clear();
        for (di, dj) in DIRS {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if ni >= 0 && nj >= 0 && (ni as usize) < n && (nj as usize) < n {
                let (ni, nj) = (ni as usize, nj as usize);
                if grid[ni][nj] == b'.' && !vis[ni][nj] {
                    buf.push((ni, nj));
                }
            }
        }
        if buf.is_empty() {
            stack.pop();
        } else {
            let (ni, nj) = buf[rng.gen_bound(buf.len())];
            vis[ni][nj] = true;
            parent[ni][nj] = (i, j);
            stack.push((ni, nj));
        }
    }
    (vis, parent)
}

fn tree_adj(
    vis: &[Vec<bool>],
    parent: &[Vec<(usize, usize)>],
    n: usize,
) -> Vec<Vec<Vec<(usize, usize)>>> {
    let mut adj = vec![vec![Vec::new(); n]; n];
    for i in 0..n {
        for j in 0..n {
            if vis[i][j] && parent[i][j].0 != usize::MAX {
                let (pi, pj) = parent[i][j];
                adj[i][j].push((pi, pj));
                adj[pi][pj].push((i, j));
            }
        }
    }
    adj
}

// Embed a trie-structured Gray-code counter into the maze: trunk = start->throne
// tree path, switches at side-branches, nested gates force the hero through ~2^K
// mask states (T ~ exponential). One bit reserved for permanent walls.
// Returns (dh, dv, sw) grids, or None if it doesn't fit (<=50 doors, enough branches).
fn build_counter(
    grid: &[Vec<u8>],
    n: usize,
    throne: (usize, usize),
    kbits: usize,
    rng: &mut impl RandomTrait,
) -> Option<(Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>)> {
    let nbrs = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (di, dj) in DIRS {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if ni >= 0 && nj >= 0 && (ni as usize) < n && (nj as usize) < n
                && grid[ni as usize][nj as usize] == b'.'
            {
                v.push((ni as usize, nj as usize));
            }
        }
        v
    };
    // DFS tree from (0,0), throne not expanded (leaf).
    let mut parent = vec![vec![(usize::MAX, usize::MAX); n]; n];
    let mut vis = vec![vec![false; n]; n];
    vis[0][0] = true;
    parent[0][0] = (0, 0);
    let mut st = vec![(0usize, 0usize)];
    while let Some(&(i, j)) = st.last() {
        if (i, j) == throne {
            st.pop();
            continue;
        }
        let unvis: Vec<(usize, usize)> = nbrs(i, j).into_iter().filter(|&(a, b)| !vis[a][b]).collect();
        if unvis.is_empty() {
            st.pop();
        } else {
            let (ni, nj) = unvis[rng.gen_bound(unvis.len())];
            vis[ni][nj] = true;
            parent[ni][nj] = (i, j);
            st.push((ni, nj));
        }
    }
    if !vis[throne.0][throne.1] {
        return None;
    }
    let in_tree = |a: (usize, usize), b: (usize, usize)| parent[a.0][a.1] == b || parent[b.0][b.1] == a;
    // tree adjacency
    let mut tadj = vec![vec![Vec::<(usize, usize)>::new(); n]; n];
    for i in 0..n {
        for j in 0..n {
            if vis[i][j] && parent[i][j] != (i, j) && parent[i][j].0 != usize::MAX {
                let (pi, pj) = parent[i][j];
                tadj[i][j].push((pi, pj));
                tadj[pi][pj].push((i, j));
            }
        }
    }
    // trunk = tree path (0,0) -> throne (parent chain)
    let mut trunk = Vec::new();
    let mut x = throne;
    loop {
        trunk.push(x);
        if x == (0, 0) {
            break;
        }
        x = parent[x.0][x.1];
    }
    trunk.reverse(); // start .. throne
    let mut on_trunk = vec![vec![false; n]; n];
    for &(i, j) in &trunk {
        on_trunk[i][j] = true;
    }
    // branch points: trunk cells (not start/throne) with a tree-child off trunk
    let mut branches: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for idx in 1..trunk.len().saturating_sub(1) {
        let c = trunk[idx];
        let mut best: Option<(usize, usize)> = None;
        let mut best_depth = -1i32;
        for &d in &tadj[c.0][c.1] {
            if on_trunk[d.0][d.1] { continue; }
            let mut dd = vec![vec![-1i32; n]; n];
            dd[c.0][c.1] = -2;
            dd[d.0][d.1] = 0;
            let mut mx = 0i32;
            let mut q = VecDeque::new();
            q.push_back(d);
            while let Some((ci, cj)) = q.pop_front() {
                for &(a, b) in &tadj[ci][cj] {
                    if dd[a][b] == -1 {
                        dd[a][b] = dd[ci][cj] + 1;
                        if dd[a][b] > mx { mx = dd[a][b]; }
                        q.push_back((a, b));
                    }
                }
            }
            if mx > best_depth { best_depth = mx; best = Some(d); }
        }
        if let Some(d) = best { branches.push((c, d)); }
    }
    let k = kbits.min(branches.len() + 1); // +1 for the free bit 0
    if k < 3 {
        return None;
    }
    let wallbit = k; // reserved: never toggled -> type 2k+1 always closed
    let mut dh = vec![vec![-1i32; n]; n];
    let mut dv = vec![vec![-1i32; n]; n];
    let mut sw = vec![vec![-1i32; n]; n];
    let set_door = |dh: &mut Vec<Vec<i32>>, dv: &mut Vec<Vec<i32>>, e: (u8, usize, usize), t: i32| {
        if e.0 == 0 {
            dh[e.1][e.2] = t;
        } else {
            dv[e.1][e.2] = t;
        }
    };
    // walls: every non-tree empty edge -> permanently closed (type 2*wallbit+1)
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] != b'.' {
                continue;
            }
            if i + 1 < n && grid[i + 1][j] == b'.' && !in_tree((i, j), (i + 1, j)) {
                set_door(&mut dh, &mut dv, (0, i, j), 2 * wallbit as i32 + 1);
            }
            if j + 1 < n && grid[i][j + 1] == b'.' && !in_tree((i, j), (i, j + 1)) {
                set_door(&mut dh, &mut dv, (1, i, j), 2 * wallbit as i32 + 1);
            }
        }
    }
    // bit 0 (flipped ~2^(K-1) times -- by far the most) goes to the deepest
    // gate-free side-branch off the hub, so each flip is an expensive detour.
    {
        let mut dd = vec![vec![-1i32; n]; n];
        for &(ti, tj) in &trunk {
            dd[ti][tj] = -2;
        }
        dd[0][0] = 0;
        let mut deep0 = (0usize, 0usize);
        let mut q = VecDeque::new();
        q.push_back((0usize, 0usize));
        while let Some((ci, cj)) = q.pop_front() {
            for &(a, b) in &tadj[ci][cj] {
                if dd[a][b] == -1 {
                    dd[a][b] = dd[ci][cj] + 1;
                    if dd[a][b] > dd[deep0.0][deep0.1] {
                        deep0 = (a, b);
                    }
                    q.push_back((a, b));
                }
            }
        }
        sw[deep0.0][deep0.1] = 0;
    }
    // branch j (0..k-2): switch toggles bit j+1, branch-edge gated bit j = 1,
    // trunk edge leaving the branch toward throne gated bit j = 0.
    for j in 0..(k - 1) {
        let (tc, scell) = branches[j];
        set_door(&mut dh, &mut dv, edge(tc, scell), 2 * j as i32 + 1);
        // push the switch to the deepest cell of the side-subtree (longer arm
        // -> larger cost per flip for this bit).
        let mut deep = scell;
        let mut dd = vec![vec![-1i32; n]; n];
        dd[scell.0][scell.1] = 0;
        dd[tc.0][tc.1] = 0; // barrier back to trunk
        let mut q = VecDeque::new();
        q.push_back(scell);
        while let Some((ci, cj)) = q.pop_front() {
            for &(a, b) in &tadj[ci][cj] {
                if dd[a][b] == -1 {
                    dd[a][b] = dd[ci][cj] + 1;
                    if dd[a][b] > dd[deep.0][deep.1] {
                        deep = (a, b);
                    }
                    q.push_back((a, b));
                }
            }
        }
        sw[deep.0][deep.1] = (j + 1) as i32;
        // trunk edge from tc toward throne
        let ti = trunk.iter().position(|&p| p == tc).unwrap();
        let nxt = trunk[ti + 1];
        set_door(&mut dh, &mut dv, edge(tc, nxt), 2 * j as i32);
    }
    // final throne gate: bit (k-1) = 1
    let tlast = trunk[trunk.len() - 2];
    set_door(&mut dh, &mut dv, edge(tlast, throne), 2 * (k - 1) as i32 + 1);
    // door budget
    let nd: usize = (0..n)
        .map(|i| (0..n).filter(|&j| dh[i][j] != -1).count() + (0..n).filter(|&j| dv[i][j] != -1).count())
        .sum();
    if nd > 50 {
        return None;
    }
    Some((dh, dv, sw))
}

// Nested switch chain over `adj`. Step i (1..=9): seal `current`'s single
// edge with closed door 2i-1, place switch i-1 at the farthest leaf x_i, then
// cut `current`. Forces the relay (0,0) -> x9 -> ... -> x1 -> throne.
// Returns (doors, switches, value) where value is the sum of the maximized
// leg lengths -- a direct proxy for the forced action count T.
fn build_chain(
    adj: &[Vec<Vec<(usize, usize)>>],
    visited: &[Vec<bool>],
    n: usize,
    throne: (usize, usize),
) -> (Vec<(u8, usize, usize, u32)>, Vec<(usize, usize, usize)>, i64) {
    let mut chain_doors: Vec<(u8, usize, usize, u32)> = Vec::new();
    let mut switches: Vec<(usize, usize, usize)> = Vec::new();
    let mut cut = vec![vec![false; n]; n];
    let mut current = throne;
    let mut value = 0i64;
    for i in 1..=9usize {
        let distc = tree_bfs(adj, &cut, n, current);
        let dist0 = if i == 9 {
            Some(tree_bfs(adj, &cut, n, (0, 0)))
        } else {
            None
        };
        let mut best: Option<(usize, usize)> = None;
        let mut best_val = -1i64;
        for x in 0..n {
            for y in 0..n {
                if !visited[x][y] || cut[x][y] || (x, y) == (0, 0) || (x, y) == current {
                    continue;
                }
                if distc[x][y] < 0 {
                    continue;
                }
                if adj[x][y].iter().filter(|&&(a, b)| !cut[a][b]).count() != 1 {
                    continue;
                }
                let mut val = distc[x][y] as i64;
                if let Some(d0) = &dist0 {
                    if d0[x][y] < 0 {
                        continue;
                    }
                    val += d0[x][y] as i64;
                }
                if val > best_val {
                    best_val = val;
                    best = Some((x, y));
                }
            }
        }
        let Some((x, y)) = best else { break };
        let Some(&nb) = adj[current.0][current.1].iter().find(|&&(a, b)| !cut[a][b]) else {
            break;
        };
        let (d, ei, ej) = edge(current, nb);
        chain_doors.push((d, ei, ej, (2 * i - 1) as u32));
        switches.push((x, y, i - 1));
        value += best_val;
        cut[current.0][current.1] = true;
        current = (x, y);
    }
    (chain_doors, switches, value)
}

// Tree edges on the unique path u -> v over `adj`.
fn tree_path(
    adj: &[Vec<Vec<(usize, usize)>>],
    n: usize,
    u: (usize, usize),
    v: (usize, usize),
) -> Vec<(u8, usize, usize)> {
    let mut par = vec![vec![(usize::MAX, usize::MAX); n]; n];
    let mut q = VecDeque::new();
    par[u.0][u.1] = u;
    q.push_back(u);
    while let Some((i, j)) = q.pop_front() {
        if (i, j) == v {
            break;
        }
        for &(a, b) in &adj[i][j] {
            if par[a][b] == (usize::MAX, usize::MAX) {
                par[a][b] = (i, j);
                q.push_back((a, b));
            }
        }
    }
    let mut edges = Vec::new();
    if par[v.0][v.1] == (usize::MAX, usize::MAX) {
        return edges;
    }
    let mut cur = v;
    while cur != u {
        let p = par[cur.0][cur.1];
        edges.push(edge(p, cur));
        cur = p;
    }
    edges
}

// Simulated annealing over spanning trees (throne kept a leaf), maximizing the
// chain leg-sum. Move = add a non-tree edge, remove a random edge on the cycle
// it closes. Returns the best tree's adjacency.
fn optimize_tree(
    mut adj: Vec<Vec<Vec<(usize, usize)>>>,
    visited: &[Vec<bool>],
    n: usize,
    throne: (usize, usize),
    all_edges: &[(u8, usize, usize)],
    rng: &mut impl RandomTrait,
    start: &Instant,
    deadline: std::time::Duration,
) -> Vec<Vec<Vec<(usize, usize)>>> {
    let endpoints = |e: (u8, usize, usize)| -> ((usize, usize), (usize, usize)) {
        let (d, i, j) = e;
        if d == 0 {
            ((i, j), (i + 1, j))
        } else {
            ((i, j), (i, j + 1))
        }
    };
    let mut tree_set: HashSet<(u8, usize, usize)> = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            for &(a, b) in &adj[i][j] {
                tree_set.insert(edge((i, j), (a, b)));
            }
        }
    }
    let mut cur = build_chain(&adj, visited, n, throne).2;
    let mut best_adj = adj.clone();
    let mut best = cur;
    let dl = deadline.as_secs_f64();
    let t_start = start.elapsed().as_secs_f64();
    while start.elapsed() < deadline {
        let add = all_edges[rng.gen_bound(all_edges.len())];
        if tree_set.contains(&add) {
            continue;
        }
        let (u, v) = endpoints(add);
        if u == throne || v == throne {
            continue;
        }
        let path = tree_path(&adj, n, u, v);
        if path.is_empty() {
            continue;
        }
        let rem = path[rng.gen_bound(path.len())];
        let (ra, rb) = endpoints(rem);
        adj[ra.0][ra.1].retain(|&x| x != rb);
        adj[rb.0][rb.1].retain(|&x| x != ra);
        adj[u.0][u.1].push(v);
        adj[v.0][v.1].push(u);
        let nv = build_chain(&adj, visited, n, throne).2;
        let delta = (nv - cur) as f64;
        let progress = ((start.elapsed().as_secs_f64() - t_start) / (dl - t_start)).clamp(0.0, 1.0);
        let temp = 200.0_f64 * (1.0f64 / 200.0).powf(progress); // 200 -> 1
        let accept = delta >= 0.0
            || (rng.gen_bound(1_000_000u64) as f64 / 1e6) < (delta / temp).exp();
        if accept {
            tree_set.remove(&rem);
            tree_set.insert(add);
            cur = nv;
            if nv > best {
                best = nv;
                best_adj = adj.clone();
            }
        } else {
            adj[u.0][u.1].retain(|&x| x != v);
            adj[v.0][v.1].retain(|&x| x != u);
            adj[ra.0][ra.1].push(rb);
            adj[rb.0][rb.1].push(ra);
        }
    }
    best_adj
}

// Minimum hero action count for a full door/switch configuration (the
// official calc_T). BFS over (K-bit switch-parity mask, row, col). 0 =
// unreachable. Buffers are reused across calls (generation-stamped `seen`)
// so each evaluation avoids reallocating/zeroing the 2^K*N^2 state array.
struct Bfs {
    n: usize,
    dist: Vec<i32>,
    seen: Vec<u32>,
    from: Vec<usize>,
    goal: usize,
    stamp: u32,
    q: VecDeque<(usize, usize, usize)>,
}

impl Bfs {
    fn new(n: usize, k: usize) -> Self {
        let size = (1usize << k) * n * n;
        Bfs {
            n,
            dist: vec![0; size],
            seen: vec![0; size],
            from: vec![0; size],
            goal: usize::MAX,
            stamp: 0,
            q: VecDeque::new(),
        }
    }

    fn calc_t(&mut self, grid: &[Vec<u8>], dh: &[Vec<i32>], dv: &[Vec<i32>], sw: &[Vec<i32>]) -> i64 {
        let n = self.n;
        let idx = |mask: usize, i: usize, j: usize| (mask * n + i) * n + j;
        let is_open = |g: i32, mask: usize| g < 0 || ((mask >> (g / 2) as usize) & 1) as i32 == (g & 1);
        self.stamp += 1;
        let g = self.stamp;
        self.goal = usize::MAX;
        self.q.clear();
        let s0 = idx(0, 0, 0);
        self.dist[s0] = 0;
        self.seen[s0] = g;
        self.from[s0] = s0;
        self.q.push_back((0, 0, 0));
        while let Some((mask, i, j)) = self.q.pop_front() {
            let cur = idx(mask, i, j);
            let d = self.dist[cur];
            if i == n - 1 && j == n - 1 {
                self.goal = cur;
                return d as i64;
            }
            for (di, dj) in DIRS {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni < 0 || nj < 0 || ni >= n as isize || nj >= n as isize {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if grid[ni][nj] == b'#' {
                    continue;
                }
                let door = if di == 1 {
                    dh[i][j]
                } else if di == -1 {
                    dh[ni][nj]
                } else if dj == 1 {
                    dv[i][j]
                } else {
                    dv[ni][nj]
                };
                if is_open(door, mask) {
                    let id = idx(mask, ni, nj);
                    if self.seen[id] != g {
                        self.seen[id] = g;
                        self.dist[id] = d + 1;
                        self.from[id] = cur;
                        self.q.push_back((mask, ni, nj));
                    }
                }
            }
            let s = sw[i][j];
            if s != -1 {
                let nmask = mask ^ (1usize << s);
                let id = idx(nmask, i, j);
                if self.seen[id] != g {
                    self.seen[id] = g;
                    self.dist[id] = d + 1;
                    self.from[id] = cur;
                    self.q.push_back((nmask, i, j));
                }
            }
        }
        0
    }

    // Geometric edges traversed on the hero's optimal path, each paired with
    // the mask he holds while crossing (from the most recent calc_t). Empty if
    // the throne was unreachable. Deduped by (edge, mask).
    fn path_crossings(&self) -> Vec<((u8, usize, usize), usize)> {
        let n = self.n;
        let mut res = Vec::new();
        if self.goal == usize::MAX {
            return res;
        }
        let decode = |id: usize| {
            let j = id % n;
            let t = id / n;
            (t / n, t % n, j) // (mask, i, j)
        };
        let mut seen = HashSet::new();
        let mut cur = self.goal;
        loop {
            let p = self.from[cur];
            if p == cur {
                break;
            }
            let (cm, ci, cj) = decode(cur);
            let (pm, pi, pj) = decode(p);
            if cm == pm && (ci, cj) != (pi, pj) {
                let e = edge((pi, pj), (ci, cj));
                if seen.insert((e, cm)) {
                    res.push((e, cm));
                }
            }
            cur = p;
        }
        res
    }
}

// "Reuse switch k" move: swap every door of type 2k<->2k+1, add a fresh closed
// door 2k+1 on `door`'s edge, and switch k on `leaf`. None if a slot is taken.
fn apply_reuse_move(
    dh: &[Vec<i32>],
    dv: &[Vec<i32>],
    sw: &[Vec<i32>],
    n: usize,
    kk: usize,
    door: (u8, usize, usize),
    leaf: (usize, usize),
) -> Option<(Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>)> {
    let (dd, di, dj) = door;
    if (dd == 0 && dh[di][dj] != -1) || (dd == 1 && dv[di][dj] != -1) || sw[leaf.0][leaf.1] != -1 {
        return None;
    }
    let (a, b) = ((2 * kk) as i32, (2 * kk + 1) as i32);
    let mut dh2 = dh.to_vec();
    let mut dv2 = dv.to_vec();
    let mut sw2 = sw.to_vec();
    for i in 0..n {
        for j in 0..n {
            for g in [&mut dh2[i][j], &mut dv2[i][j]] {
                if *g == a {
                    *g = b;
                } else if *g == b {
                    *g = a;
                }
            }
        }
    }
    if dd == 0 {
        dh2[di][dj] = b;
    } else {
        dv2[di][dj] = b;
    }
    sw2[leaf.0][leaf.1] = kk as i32;
    Some((dh2, dv2, sw2))
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let t0 = Instant::now();
    let n = input.read_size();
    let _m = input.read_size();
    let k = input.read_size();
    let grid: Vec<Vec<u8>> = (0..n).map(|_| input.next_token().unwrap()).collect();
    let throne = (n - 1, n - 1);

    let empty = |i: isize, j: isize| -> bool {
        i >= 0 && j >= 0 && (i as usize) < n && (j as usize) < n && grid[i as usize][j as usize] == b'.'
    };
    let nbrs = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (di, dj) in DIRS {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if empty(ni, nj) {
                v.push((ni as usize, nj as usize));
            }
        }
        v
    };

    // Build many random-DFS spanning trees (throne forced to a leaf) and keep
    // the one whose nested chain yields the largest leg-sum -- i.e. select
    // directly on the forced action count T rather than a diameter proxy.
    let mut rng = Random::new_with_seed(0x9E3779B97F4A7C15);
    let mut best: Option<(
        Vec<Vec<bool>>,
        Vec<Vec<(usize, usize)>>,
        Vec<Vec<Vec<(usize, usize)>>>,
        i64,
    )> = None;
    for _ in 0..TREE_TRIES {
        let (vis, par) = random_dfs(&grid, n, throne, &mut rng);
        let adj = tree_adj(&vis, &par, n);
        let value = build_chain(&adj, &vis, n, throne).2;
        if best.as_ref().map_or(true, |x| value > x.3) {
            best = Some((vis, par, adj, value));
        }
    }
    let (visited, _parent, adj0, _) = best.unwrap();

    // All grid edges between two reached cells; the tree is a spanning subset.
    let mut all_edges: Vec<(u8, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if visited[i][j] {
                if i + 1 < n && visited[i + 1][j] {
                    all_edges.push((0, i, j));
                }
                if j + 1 < n && visited[i][j + 1] {
                    all_edges.push((1, i, j));
                }
            }
        }
    }

    // Improve the tree by simulated annealing on edge swaps (chain leg-sum).
    let adj = optimize_tree(
        adj0,
        &visited,
        n,
        throne,
        &all_edges,
        &mut rng,
        &t0,
        std::time::Duration::from_millis(300),
    );

    // Tree edge set; walls = reached-reached edges not in the tree.
    let mut tree_set: HashSet<(u8, usize, usize)> = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            for &(a, b) in &adj[i][j] {
                tree_set.insert(edge((i, j), (a, b)));
            }
        }
    }
    let _ = &nbrs;

    // Emit the chain for the chosen tree.
    let (chain_doors, switches, _) = build_chain(&adj, &visited, n, throne);

    // Permanent walls: every non-tree reached edge becomes a closed door of
    // type 19 (switch 9 is never placed), keeping the hero on the tree.
    let walls: Vec<(u8, usize, usize)> = all_edges
        .iter()
        .copied()
        .filter(|e| !tree_set.contains(e))
        .collect();

    // Assemble the configuration as door/switch grids.
    let mut dh = vec![vec![-1i32; n]; n]; // door (i,j)-(i+1,j)
    let mut dv = vec![vec![-1i32; n]; n]; // door (i,j)-(i,j+1)
    let mut sw = vec![vec![-1i32; n]; n];
    for &(d, i, j, g) in &chain_doors {
        if d == 0 {
            dh[i][j] = g as i32;
        } else {
            dv[i][j] = g as i32;
        }
    }
    for &(d, i, j) in &walls {
        if d == 0 {
            dh[i][j] = 19;
        } else {
            dv[i][j] = 19;
        }
    }
    for &(p, q, s) in &switches {
        sw[p][q] = s as i32;
    }
    let door_count = |dh: &[Vec<i32>], dv: &[Vec<i32>]| -> usize {
        (0..n)
            .map(|i| (0..n).filter(|&j| dh[i][j] != -1).count() + (0..n).filter(|&j| dv[i][j] != -1).count())
            .sum()
    };

    // Post-processing: greedily try "reuse switch k" moves -- swap all doors of
    // types 2k<->2k+1, add a fresh closed door 2k+1 on an interior tree edge
    // (re-sealing a forced-path segment), and a fresh switch k on a leaf. Keep
    // the move only if the throne stays reachable and T grows. Search by
    // sampling candidates within the time budget.
    let tree_edges: Vec<(u8, usize, usize)> = tree_set.iter().copied().collect();
    let leaves: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .filter(|&(i, j)| visited[i][j] && adj[i][j].len() == 1 && (i, j) != (0, 0) && (i, j) != throne)
        .collect();

    // The chain config (dh,dv,sw) is the always-valid safety floor. The real
    // solution is the embedded Gray-code counter: try many randomized variants
    // within the time budget and keep the one with the largest forced T.
    let mut bfs = Bfs::new(n, k);
    let budget = std::time::Duration::from_millis(1700);
    let mut best_t = bfs.calc_t(&grid, &dh, &dv, &sw);
    let _ = (&tree_edges, &leaves, door_count, apply_reuse_move);
    while t0.elapsed() < budget {
        if let Some((cdh, cdv, csw)) = build_counter(&grid, n, throne, k - 1, &mut rng) {
            let ct = bfs.calc_t(&grid, &cdh, &cdv, &csw);
            if ct > best_t {
                best_t = ct;
                dh = cdh;
                dv = cdv;
                sw = csw;
            }
        }
    }

    // Emit final configuration.
    let mut out_doors: Vec<(u8, usize, usize, i32)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if dh[i][j] != -1 {
                out_doors.push((0, i, j, dh[i][j]));
            }
            if dv[i][j] != -1 {
                out_doors.push((1, i, j, dv[i][j]));
            }
        }
    }
    writeln!(out, "{}", out_doors.len()).unwrap();
    for (d, i, j, g) in &out_doors {
        writeln!(out, "{} {} {} {}", d, i, j, g).unwrap();
    }
    let mut out_sw: Vec<(usize, usize, i32)> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if sw[i][j] != -1 {
                out_sw.push((i, j, sw[i][j]));
            }
        }
    }
    writeln!(out, "{}", out_sw.len()).unwrap();
    for (p, q, s) in &out_sw {
        writeln!(out, "{} {} {}", p, q, s).unwrap();
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
