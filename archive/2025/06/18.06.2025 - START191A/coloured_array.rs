//{"name":"Coloured Array","group":"CodeChef - START191A","url":"https://www.codechef.com/START191A/problems/COLARR7","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n2 2 1 1 3 3\n3\n1 1 1 1 1 1\n3\n1 2 1 2 1 2\n","output":"0\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(2 * n).dec();

    let mut good = 0;
    let mut occupied = BitSet::new(n);
    for i in 0..n {
        if a[2 * i] == a[2 * i + 1] && !occupied[a[2 * i]] {
            good += 1;
            occupied.set(a[2 * i]);
        }
    }
    let mut graph = Graph::new(2 * n + 2);
    let source = 2 * n;
    let sink = 2 * n + 1;
    for i in 0..n {
        graph.add_edge(FlowEdge::new(n + i, sink, 1));
        graph.add_edge(FlowEdge::new(a[2 * i], n + i, 1));
        graph.add_edge(FlowEdge::new(a[2 * i + 1], n + i, 1));
        if !occupied[i] {
            graph.add_edge(FlowEdge::new(source, i, 1));
        }
    }
    out.print_line(2 * n - 2 * good - graph.max_flow(source, sink));
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
