//{"name":"D. MEX Replacement on Tree","group":"Codeforces - Codeforces Round 1093 (Div. 1)","url":"https://codeforces.com/contest/2219/problem/D","interactive":false,"timeLimit":3500,"tests":[{"input":"7\n1\n0\n3\n1 0 2\n1 2\n1 3\n6\n1 4 5 2 0 3\n1 4\n1 5\n6 2\n2 5\n2 3\n5\n1 2 3 0 4\n1 2\n2 3\n3 4\n4 5\n10\n9 8 7 1 3 2 5 4 6 0\n6 10\n3 1\n8 7\n4 2\n2 8\n7 5\n10 9\n3 9\n6 4\n9\n8 4 0 6 5 7 3 1 2\n8 3\n4 2\n4 6\n9 3\n7 6\n1 5\n8 5\n9 2\n7\n1 5 2 3 6 0 4\n7 6\n4 3\n7 2\n5 1\n2 4\n6 5\n","output":"1\n4\n12\n9\n30\n26\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::treap::Tree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut base = 0;
    let mut delta = 0;
    struct Node {
        self_val: usize,
        sum: usize,
        key: (usize, usize),
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.self_val + left.map_or(0, |v| v.sum) + right.map_or(0, |v| v.sum)
        }
    }
    impl OrdPayload for Node {
        type Key = (usize, usize);

        fn key(&self) -> &Self::Key {
            &self.key
        }
    }

    let mut set = (0..n).collect::<BTreeSet<_>>();
    let mut dfs =
        RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Tree<Node>, Tree<Node>) {
            let no_return = set.first().copied().unwrap() == p[vert];
            set.remove(&p[vert]);
            let first = set.first().copied().unwrap_or(n);
            base += first;
            let second = set.iter().nth(1).copied().unwrap_or(n);
            let mut res = Tree::single(Node {
                self_val: second,
                sum: second,
                key: (second, vert),
            });
            let mut base_tree = Tree::single(Node {
                self_val: first,
                sum: first,
                key: (first, vert),
            });
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let (call, call_base) = f.call(e.to(), vert);
                res = Tree::union(res, call);
                base_tree = Tree::union(base_tree, call_base);
            }
            let (mut front, back) = res.split(&(p[vert], 0));
            if p[vert] > first {
                let mut cur = front.payload().map_or(0, |node| node.sum) - front.size() * first
                    + back.size() * (p[vert] - first);
                let (front_base, mut back_base) = base_tree.split(&(p[vert], 0));
                let sub =
                    back_base.payload().map_or(0, |node| node.sum) - p[vert] * back_base.size();
                cur = cur.saturating_sub(sub);
                delta.maxim(cur);
                base_tree = Tree::merge(front_base, back_base);
            }
            set.insert(p[vert]);
            if no_return {
                (Tree::new(), base_tree)
            } else {
                (Tree::merge(front, back), base_tree)
            }
        });
    dfs.call(0, n);
    out.print_line(base + delta);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
