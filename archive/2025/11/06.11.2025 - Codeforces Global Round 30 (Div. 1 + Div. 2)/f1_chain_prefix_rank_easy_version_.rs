//{"name":"F1. Chain Prefix Rank (Easy Version)","group":"Codeforces - Codeforces Global Round 30 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2164/problem/F1","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n5\n1 2 3 4\n0 1 2 3 4\n5\n1 2 1 1\n0 1 0 0 0\n8\n1 1 3 3 4 5 7\n0 0 1 0 1 3 3 1\n","output":"1\n6\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Tree;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value::{DynamicValue, Value};
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::One;
use algo_lib::{dynamic_value, value_ref};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let f = input.read_size_vec(n - 1).dec();
    let a = input.read_size_vec(n);

    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        graph.add_edge(BiEdge::new(f[i], i + 1));
    }
    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n + 1);
    value_ref!(C: Combinations<Mod> = c);
    dynamic_value!(ANS: Mod = Mod::one());
    struct Node {
        key: usize,
        delta_key: usize,
        qty: usize,
    }
    impl Payload for Node {
        const NEED_ACCUMULATE: bool = true;

        fn accumulate(&mut self, delta: &Self) {
            self.key -= delta.delta_key;
            self.delta_key += delta.delta_key;
        }

        fn reset_delta(&mut self) {
            self.delta_key = 0;
        }
    }
    impl OrdPayload for Node {
        type Key = usize;

        fn key(&self) -> &Self::Key {
            &self.key
        }

        fn union(a: Self, b: Self) -> Self {
            ANS::set(ANS::val() * C::with(|c| c.c(a.qty + b.qty, a.qty)));
            Self {
                key: a.key,
                delta_key: 0,
                qty: a.qty + b.qty,
            }
        }
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Tree<Node> {
        let mut res = Tree::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            res = Tree::union(res, call);
        }
        let q_at = res
            .range(&a[vert]..=&a[vert])
            .detach()
            .payload()
            .map(|node| node.qty)
            .unwrap_or(0);
        let q_at1 = res
            .range(&(a[vert] + 1)..=&(a[vert] + 1))
            .detach()
            .payload()
            .map(|node| node.qty)
            .unwrap_or(0);
        res.range(&(a[vert] + 2)..).push(&Node {
            key: 0,
            delta_key: 1,
            qty: 0,
        });
        res.insert(Node {
            key: a[vert],
            delta_key: 0,
            qty: q_at + q_at1 + 1,
        });
        res
    });
    dfs.call(0, n);

    out.print_line(ANS::val());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
