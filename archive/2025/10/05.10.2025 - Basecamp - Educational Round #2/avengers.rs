//{"name":"Avengers","group":"Eolymp - Basecamp - Educational Round #2","url":"https://basecamp.eolymp.com/en/compete/b0f0h1n10h679euo6gn2c7821o/problem/8","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 2\n2 3\n","output":"-1\n"},{"input":"3 2\n1 2\n1 3\n","output":"2 3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let vu = input.read_size_pair_vec(m).dec();

    if n < 3 {
        out.print_line(-1);
        return;
    }

    let graph = Graph::with_edges(n, &vu);
    let order = graph.topological_sort().unwrap();

    for (a, b) in order.consecutive_iter_copy() {
        let mut found = false;
        for e in &graph[a] {
            if e.to() == b {
                found = true;
                break;
            }
        }
        if !found {
            let mut c = 0;
            while c == a || c == b {
                c += 1;
            }
            out.print_line((a + 1, b + 1, c + 1));
            return;
        }
    }
    out.print_line(-1);
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
