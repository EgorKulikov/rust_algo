//{"name":"E - Part-Time Job Shift Assignment","group":"AtCoder - AtCoder Weekday Contest 0013 Beta","url":"https://atcoder.jp/contests/awc0013/tasks/awc0013_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n2 1 2\n2 2 3\n2 1 3\n","output":"3\n"},{"input":"4 5\n3 1 2 3\n2 2 4\n1 5\n2 3 4\n","output":"4\n"},{"input":"6 8\n4 1 2 3 4\n3 2 5 6\n2 1 3\n3 4 7 8\n2 6 8\n0\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let c = input
        .iter::<Vec<usize>>()
        .take(n)
        .map(Vec::dec)
        .collect::<Vec<_>>();

    let mut graph = Graph::new(n + m + 2);
    let source = n + m;
    let sink = n + m + 1;
    for i in 0..n {
        graph.add_edge(FlowEdge::new(source, i, 1));
        for j in c[i].copy_iter() {
            graph.add_edge(FlowEdge::new(i, n + j, 1));
        }
    }
    for i in 0..m {
        graph.add_edge(FlowEdge::new(n + i, sink, 1));
    }
    out.print_line(graph.max_flow(source, sink));
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
