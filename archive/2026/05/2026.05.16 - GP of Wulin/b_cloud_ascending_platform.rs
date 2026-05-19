//{"name":"B. Cloud-Ascending Platform","group":"Universal Cup - GP of Wulin","url":"https://contest.ucup.ac/contest/3749/problem/18120","interactive":false,"timeLimit":1000,"tests":[{"input":"5 6 6\n1 2 1\n2 3 2\n2 4 4\n2 5 8\n1 3 16\n4 5 32\n1 3\n2 4\n3 5\n4 6\n2 6\n1 6\n","output":"7\n14\n28\n56\n18\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    weight: u64,
}

struct GraphIter<'a> {
    graph: &'a Graph,
    edge_id: usize,
}

impl Iterator for GraphIter<'_> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.edge_id == usize::MAX {
            return None;
        }
        let edge_id = self.edge_id;
        let edge = self.graph.edges[edge_id];
        self.edge_id = self.graph.next[edge_id];
        Some(edge)
    }
}

struct Graph {
    first: Vec<usize>,
    next: Vec<usize>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            first: vec![usize::MAX; n],
            next: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.first.fill(usize::MAX);
        self.next.clear();
        self.edges.clear();
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: u64) {
        self.next.push(self.first[from]);
        self.first[from] = self.next.len() - 1;
        self.edges.push(Edge { to, weight });
        self.next.push(self.first[to]);
        self.first[to] = self.next.len() - 1;
        self.edges.push(Edge { to: from, weight });
    }

    fn pop_edge(&mut self, from: usize, to: usize) {
        debug_assert_eq!(self.first[to], self.next.len() - 1);
        self.first[to] = self.next.pop().unwrap();
        self.edges.pop();
        debug_assert_eq!(self.first[from], self.next.len() - 1);
        self.first[from] = self.next.pop().unwrap();
        self.edges.pop();
    }

    fn iter(&self, vert: usize) -> GraphIter<'_> {
        GraphIter {
            graph: self,
            edge_id: self.first[vert],
        }
    }

    fn vertex_count(&self) -> usize {
        self.first.len()
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    // let mut tt = TimeTracker::new();
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let edges = input.read_vec::<(usize, usize, u64)>(m).dec();
    let queries = input.read_size_pair_vec(q).dec();

    let mut by_end = vec![Vec::new(); m];
    for (i, (l, r)) in queries.copy_enumerate() {
        by_end[r].push((l, i));
    }
    for i in 0..m {
        by_end[i].sort();
        by_end[i].reverse();
    }

    const BUBEN: usize = 333;
    // let mut dsu = DSU::new(n);
    let mut fup = vec![0; n];
    let mut tin = vec![0; n];
    // let mut fup = FastClearArr::new();
    // let mut tin = FastClearArr::new();
    let mut used = BitSet::new(n);
    let mut bridges = |graph: &Graph, need_vec: bool| -> Result {
        let n = graph.vertex_count();
        let mut timer = 0;
        tin.fill(0);
        fup.fill(0);
        // tin.clear();
        // fup.clear();
        used.fill(false);
        let mut ans = if need_vec {
            Result::Vec(Vec::new())
        } else {
            Result::Sum(0)
        };
        let mut stack = Vec::new();
        for i in 0..n {
            if !used[i] {
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    used.set(vert);
                    stack.push(vert);
                    tin[vert] = timer;
                    fup[vert] = timer;
                    timer += 1;
                    let mut first = true;
                    for e in graph.iter(vert) {
                        if e.to == prev && first {
                            first = false;
                            continue;
                        }
                        let to = e.to;
                        if used[to] {
                            fup[vert].minim(tin[to]);
                        } else {
                            f.call(to, vert);
                            let cand = fup[to];
                            fup[vert].minim(cand);
                            if fup[to] > tin[vert] {
                                ans.add(vert, to, e.weight);
                                let mut cur = to;
                                while stack.last() != Some(&to) {
                                    let next = stack.pop().unwrap();
                                    ans.add(cur, next, 0);
                                    cur = next;
                                }
                                if cur != to {
                                    ans.add(cur, to, 0);
                                }
                                stack.pop();
                            }
                        }
                    }
                });
                dfs.call(i, i);
                let mut cur = i;
                while stack.last() != Some(&i) {
                    let next = stack.pop().unwrap();
                    ans.add(cur, next, 0);
                    cur = next;
                }
                if cur != i {
                    ans.add(cur, i, 0);
                }
                stack.pop();
            }
        }
        ans
    };

    let mut graph = Graph::new_linked(n);
    let mut ans = vec![0; q];
    // let mut last = vec![0; n];
    for i in (0..m).step_by(BUBEN) {
        graph.clear();
        for j in i..m {
            graph.add_edge(edges[j].0, edges[j].1, edges[j].2);
            if (j + 1) % BUBEN == 0 {
                let bridges = bridges(&graph, true);
                let bridges = match bridges {
                    Result::Vec(vec) => vec,
                    Result::Sum(_) => unreachable!(),
                };
                /*let bridges_set = bridges
                    .copy_iter()
                    .map(|(u, v, _)| (u, v))
                    .collect::<FxHashSet<_>>();
                dsu.clear();
                for (from, e) in graph.edges() {
                    let to = e.to;
                    if !bridges_set.contains(&(from, to)) && !bridges_set.contains(&(to, from)) {
                        dsu.union(from, to);
                    }
                }*/
                graph.clear();
                for (a, b, w) in bridges {
                    graph.add_edge(a, b, w);
                }
                /*for i in 0..n {
                    last[i] = i;
                }
                for i in 0..n {
                    let par = dsu.find(i);
                    if par != i {
                        graph.add_edge(i, last[par], 0);
                        last[par] = i;
                    }
                }
                for i in 0..n {
                    if last[i] != i {
                        graph.add_edge(i, last[i], 0);
                    }
                }*/
            }
            while let Some(&(l, id)) = by_end[j].last() {
                if l > i {
                    break;
                }
                for j in l..i {
                    graph.add_edge(edges[j].0, edges[j].1, edges[j].2);
                }
                ans[id] = match bridges(&graph, false) {
                    Result::Vec(_) => unreachable!(),
                    Result::Sum(sum) => sum,
                };
                for j in (l..i).rev() {
                    graph.pop_edge(edges[j].0, edges[j].1);
                }
                by_end[j].pop();
            }
        }
    }
    graph.clear();
    for i in 0..m {
        let mut last = i + 1;
        for (l, id) in by_end[i].copy_iter() {
            while last > l {
                last -= 1;
                graph.add_edge(edges[last].0, edges[last].1, edges[last].2);
            }
            ans[id] = match bridges(&graph, false) {
                Result::Vec(_) => unreachable!(),
                Result::Sum(sum) => sum,
            };
        }
        if !by_end[i].is_empty() {
            graph.clear();
        }
    }
    out.print_per_line(&ans);

    /*dynamic_value!(N: usize = n);
    value_ref!(D: DSU = DSU::new(n));
    value_ref!(Last: Vec<usize> = vec![0; n]);
    value_ref!(Gr: Graph = Graph::new_linked(n));
    #[derive(Default, Clone)]
    struct Node {
        edges: Vec<(usize, usize, u64)>,
    }

    fn solve_for() -> Node {
        let bridges = bridges(true);
        let bridges = match bridges {
            Result::Vec(vec) => vec,
            Result::Sum(_) => unreachable!(),
        };
        let bridges_set = bridges
            .copy_iter()
            .map(|(u, v, _)| (u, v))
            .collect::<FxHashSet<_>>();
        D::with_mut(|dsu| {
            dsu.clear();
            Gr::with_mut(|graph| {
                for (from, e) in graph.edges() {
                    let to = e.to;
                    if !bridges_set.contains(&(from, to)) && !bridges_set.contains(&(to, from)) {
                        dsu.union(from, to);
                    }
                }
                let mut edges = Vec::new();
                for (a, b, w) in bridges {
                    edges.push((a, b, w));
                }
                Last::with_mut(|last| {
                    for i in 0..N::val() {
                        last[i] = i;
                    }
                    for i in 0..N::val() {
                        let par = dsu.find(i);
                        if par != i {
                            edges.push((i, last[par], 0));
                            last[par] = i;
                        }
                    }
                    for i in 0..N::val() {
                        if last[i] != i {
                            edges.push((i, last[i], 0));
                        }
                    }
                    Node { edges }
                })
            })
        })
    }

    value_ref!(Tin: Vec<usize> = vec![0; n]);
    value_ref!(Fup: Vec<usize> = vec![0; n]);*/

    enum Result {
        Vec(Vec<(usize, usize, u64)>),
        Sum(u64),
    }

    impl Result {
        fn add(&mut self, a: usize, b: usize, w: u64) {
            match self {
                Result::Vec(vec) => vec.push((a, b, w)),
                Result::Sum(sum) => *sum = sum.wrapping_add(w),
            }
        }
    }

    /*impl SegmentTreeNode for Node {
        fn join(left: &Self, right: &Self) -> Self {
            Gr::with_mut(|graph| {
                graph.clear();
                for &(u, v, w) in left.edges.iter() {
                    graph.add_edge(u, v, w);
                }
                for &(u, v, w) in right.edges.iter() {
                    graph.add_edge(u, v, w);
                }
            });
            solve_for()
        }
    }

    let mut st = SegmentTree::with_gen(m, |i| Node {
        edges: vec![edges[i]],
    });
    tt.milestone("precalc");

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        Gr::with_mut(|graph| {
            graph.clear();
            st.for_each(l..r, |_, node| {
                for &(u, v, w) in &node.edges {
                    graph.add_edge(u, v, w);
                }
            });
        });
        let bridges = bridges(false);
        let res = match bridges {
            Result::Vec(_) => unreachable!(),
            Result::Sum(sum) => sum,
        };
        out.print_line(res);
    }
    tt.milestone("solve queries");*/
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
