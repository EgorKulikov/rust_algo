//{"name":"Dynamic Tree Vertex Add Subtree Sum","group":"Library Checker","url":"https://judge.yosupo.jp/problem/dynamic_tree_vertex_add_subtree_sum","interactive":false,"timeLimit":5000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::euler_tour_tree::EulerTourForest;
use algo_lib::collections::payload::Payload;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1);

    #[derive(Default)]
    struct Node {
        val: i64,
        sum: i64,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.val + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
        }
    }
    let mut etf = EulerTourForest::new();
    for i in 0..n {
        etf.add_node(Node {
            val: a[i],
            sum: a[i],
        });
    }
    for (u, v) in edges {
        etf.add_edge(u, v, Node::default(), Node::default());
    }

    for _ in 0..q {
        let t = input.read_int();
        match t {
            0 => {
                let u = input.read_size();
                let v = input.read_size();
                let w = input.read_size();
                let x = input.read_size();
                etf.remove_edge(u, v);
                etf.add_edge(w, x, Node::default(), Node::default());
            }
            1 => {
                let p = input.read_size();
                let x = input.read_long();
                etf.with_node_mut(p, |node| {
                    node.val += x;
                    node.sum += x;
                });
            }
            2 => {
                let v = input.read_size();
                let p = input.read_size();
                out.print_line(etf.with_subtree(v, p, |node| node.sum));
            }
            _ => unreachable!(),
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

mod tester {
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{run, TASK_TYPE};
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::{Random, RandomTrait};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use tester::classic::default_checker;
use tester::interactive::std_interactor;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

const PRINT_LIMIT: usize = 1000;

fn interact(mut sol_input: Input, mut sol_output: Output, mut input: Input) -> Result<(), String> {
    Ok(())
}

fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
    Ok(())
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = usize;

    fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
        Box::new(1..)
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new();
        let n = r.gen_range(2..=6);
        let q = 4;
        out.print_line((n, q));
        out.print_line(Vec::with_gen(n, |_| r.gen_range(0..10)));
        let mut edges = Vec::new();
        let mut dsu = DSU::new(n);
        for _ in 0..n - 1 {
            loop {
                let u = r.gen_range(0..n);
                let v = r.gen_range(0..n);
                if dsu.union(u, v) {
                    edges.push((u, v));
                    break;
                }
            }
        }
        out.print_per_line(&edges);
        for _ in 0..q {
            let t = r.gen_range(0..3);
            match t {
                0 => {
                    let (u, v) = edges.swap_remove(r.gen_range(0..edges.len()));
                    let mut dsu = DSU::new(n);
                    for &(u, v) in &edges {
                        dsu.union(u, v);
                    }
                    loop {
                        let w = r.gen_range(0..n);
                        let x = r.gen_range(0..n);
                        if dsu.union(w, x) {
                            edges.push((w, x));
                            out.print_line((t, u, v, w, x));
                            break;
                        }
                    }
                }
                1 => {
                    let p = r.gen_range(0..n);
                    let x = r.gen_range(0..10);
                    out.print_line((t, p, x));
                }
                2 => {
                    let (u, v) = edges[r.gen_range(0..edges.len())];
                    if r.gen_bool() {
                        out.print_line((t, u, v));
                    } else {
                        out.print_line((t, v, u));
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        let n = input.read_size();
        let q = input.read_size();
        let mut a = input.read_int_vec(n);
        let mut edges = input.read_size_pair_vec(n - 1);

        for _ in 0..q {
            let t = input.read_int();
            match t {
                0 => {
                    let u = input.read_size();
                    let v = input.read_size();
                    let w = input.read_size();
                    let x = input.read_size();
                    let pos = edges
                        .copy_find((u, v))
                        .unwrap_or_else(|| edges.copy_find((v, u)).unwrap());
                    edges[pos] = (w, x);
                }
                1 => {
                    let p = input.read_size();
                    let x = input.read_int();
                    a[p] += x;
                }
                2 => {
                    let graph = Graph::with_biedges(n, &edges);
                    let mut ans = 0;
                    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                        ans += a[vert];
                        for e in &graph[vert] {
                            if e.to() == prev {
                                continue;
                            }
                            f.call(e.to(), vert);
                        }
                    });
                    let v = input.read_size();
                    let p = input.read_size();
                    dfs.call(v, p);
                    out.print_line(ans);
                }
                _ => unreachable!(),
            }
        }
        true
    }
}

struct MaxTest;

impl GeneratedTestSet for MaxTest {
    type TestId = usize;

    fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
        Box::new(1..=1)
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new();
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./dynamic_tree_vertex_add_subtree_sum";
    let tl = 5000;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
        }
        crate::TaskType::Classic => {
            Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
            // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
        }
    };
    let passed = tester.test_samples();
    // tester.test_generated("Max test", true, MaxTest);
    // tester.test_generated("Stress test", false, StressTest);
    passed
}
}
#[test]
fn dynamic_tree_vertex_add_subtree_sum() {
    assert!(tester::run_tests());
}
