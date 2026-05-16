//{"name":"coderun18","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
// use algo_lib::graph::min_cost_flow::MinCostFlow;
use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(m);

    let mut graph = Graph::new(n + m + 3);
    let source = n + m;
    let sink = n + m + 1;
    let special_sink = n + m + 2;
    let cap = (n / m) as i64;
    for i in 0..n {
        graph.add_edge(WeightedFlowEdge::new(source, i, 0, 1));
        for j in 0..m {
            let d = a[i].upper_div(b[j]);
            graph.add_edge(WeightedFlowEdge::new(i, n + j, b[j] * d - a[i], 1));
        }
    }
    for i in 0..m {
        graph.add_edge(WeightedFlowEdge::new(n + i, sink, 0, cap));
        graph.add_edge(WeightedFlowEdge::new(n + i, special_sink, 0, 1));
    }
    graph.add_edge(WeightedFlowEdge::new(special_sink, sink, 0, (n % m) as i64));
    let flow = graph.min_cost_max_flow_slow(source, sink);
    assert_eq!(flow.flow, n as i64);
    out.print_line(flow.cost);
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
