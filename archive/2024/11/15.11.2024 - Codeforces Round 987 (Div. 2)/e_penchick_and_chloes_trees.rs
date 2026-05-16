//{"name":"E. Penchick and Chloe's Trees","group":"Codeforces - Codeforces Round 987 (Div. 2)","url":"https://codeforces.com/contest/2031/problem/E","interactive":false,"timeLimit":3500,"tests":[{"input":"5\n6\n1 2 2 1 1\n15\n1 1 2 2 3 3 4 4 5 5 6 6 7 7\n5\n1 2 2 2\n7\n1 1 2 1 1 2\n10\n1 1 1 2 2 2 4 3 3\n","output":"2\n3\n3\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPenchickAndChloesTrees"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n - 1).dec();

    let graph = Graph::new(n).do_with(|g| {
        p.enumerate().for_each(|(i, x)| {
            g.add_edge(BiEdge::new(x, i + 1));
        })
    });
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
        let mut calls = Vec::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            calls.push(f.call(e.to(), vert) + 1);
        }
        if calls.is_empty() {
            return 0;
        }
        calls.sort_by_key(|&x| Reverse(x));
        for i in calls[0].. {
            let mut have = 2;
            let mut at = i;
            let mut good = true;
            for j in calls.indices() {
                while at > calls[j] {
                    have *= 2;
                    at -= 1;
                    if j + have >= calls.len() {
                        break;
                    }
                }
                if j + have >= calls.len() {
                    break;
                }
                if have == 0 {
                    good = false;
                    break;
                }
                have -= 1;
            }
            if good {
                return i;
            }
        }
        unreachable!();
    });
    out.print_line(dfs.call(0, 0));
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
