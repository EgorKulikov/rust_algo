//{"name":"J. Tree-mendous Transmission","group":"Universal Cup - GP of China","url":"https://contest.ucup.ac/contest/3295/problem/16337","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n1 2\n2 3\n5\n1 2\n2 3\n2 4\n2 5\n7\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n","output":"1\n1 3\n2\n4 5\n1 3\n3\n7 7\n1 6\n4 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut ans = Vec::new();
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Option<usize> {
        let mut res = None;
        let mut seen = false;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            if let Some(call) = call {
                seen = true;
                if let Some(val) = res {
                    ans.push((val, call));
                    res = None;
                } else {
                    res = Some(call);
                }
            }
        }
        if !seen {
            Some(vert)
        } else {
            res
        }
    });
    if let Some(res) = dfs.call(0, n) {
        ans.push((0, res));
    }
    ans.reverse();
    out.print_line(ans.len());
    out.print_per_line(&ans.inc());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
