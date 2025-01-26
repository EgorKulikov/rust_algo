//{"name":"E1. The Game (Easy Version)","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/E1","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n4\n2 2 4 3\n1 2\n1 3\n2 4\n5\n1 2 3 4 5\n1 2\n2 3\n3 4\n4 5\n3\n1 2 3\n1 2\n1 3\n5\n3 1 3 4 5\n1 2\n2 3\n3 4\n4 5\n10\n1 2 3 2 4 3 3 4 4 3\n1 4\n4 6\n7 4\n6 9\n6 5\n7 8\n1 2\n2 3\n2 10\n","output":"2\n0\n2\n2\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let w = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let lca = graph.lca();
    let by_weight = by_index(&w);
    let mut old_lca = None;
    for i in (1..=n).rev() {
        let v = &by_weight[i];
        if v.is_empty() {
            continue;
        }
        let mut new_lca = v[0];
        for u in v {
            new_lca = lca.lca(new_lca, *u);
            if let Some(old_lca) = old_lca {
                if lca.lca(old_lca, *u) != *u {
                    out.print_line(u + 1);
                    return;
                }
            }
        }
        if let Some(old) = old_lca {
            old_lca = Some(lca.lca(old, new_lca));
        } else {
            old_lca = Some(new_lca);
        }
    }
    out.print_line(0);
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
//END MAIN
