//{"name":"Mex-P Tree (Hard)","group":"CodeChef - START176A","url":"https://www.codechef.com/START176A/problems/MPTREE","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3\n4 6 12\n1 3\n1 2\n5\n18 10 1 420 90\n2 3\n4 1\n1 3\n5 3\n","output":"9 11 11\n16 11 10 22 15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::euler_tour_tree::EulerTourForest;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::payload::Payload;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::cmp::Reverse;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::primes::sieve::primes;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let mut edges = input.read_size_pair_vec(n - 1).dec();

    let p = primes(100);
    struct Node {
        sum: usize,
        delta: usize,
        size: usize,
        self_size: usize,
    }
    impl Payload for Node {
        const NEED_ACCUMULATE: bool = true;
        const NEED_UPDATE: bool = true;

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
        fn need_push_down(&self) -> bool {
            self.delta != 0
        }
        fn accumulate(&mut self, delta: &Self) {
            self.sum += delta.delta;
            self.delta += delta.delta;
        }
        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.size = self.self_size + left.map_or(0, |l| l.size) + right.map_or(0, |r| r.size);
        }
    }
    let mut et = EulerTourForest::new();
    let mex = |a: usize| -> usize {
        for p in p.copy_iter() {
            if a % p != 0 {
                return p;
            }
        }
        unreachable!()
    };
    for i in 0..n {
        et.add_node(Node {
            sum: mex(a[i]),
            delta: 0,
            size: 1,
            self_size: 1,
        });
    }
    let edge_mex = |i: usize, j: usize| -> usize { mex(gcd(a[i], a[j])) };
    edges.sort_by_key(|&(u, v)| Reverse(edge_mex(u, v)));
    for (u, v) in edges {
        let mex = edge_mex(u, v);
        let u_size = et.with_component(u, |x| x.size);
        let v_size = et.with_component(v, |x| x.size);
        et.with_component_mut(u, |x| {
            x.sum += v_size * mex;
            x.delta += v_size * mex
        });
        et.with_component_mut(v, |x| {
            x.sum += u_size * mex;
            x.delta += u_size * mex
        });
        et.add_edge(
            u,
            v,
            Node {
                sum: 0,
                delta: 0,
                size: 0,
                self_size: 0,
            },
            Node {
                sum: 0,
                delta: 0,
                size: 0,
                self_size: 0,
            },
        );
    }
    out.print_line_iter((0..n).map(|i| et.with_node(i, |x| x.sum)));
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
        TaskType::Classic => input.is_empty(),
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
