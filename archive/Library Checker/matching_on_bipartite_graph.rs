//{"name":"Matching on Bipartite Graph","group":"Library Checker","url":"https://judge.yosupo.jp/problem/bipartitematching","interactive":false,"timeLimit":5000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::fast_max_flow::FastMaxFlow;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_size();
    let r = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m);

    let mut graph = Graph::new(l + r + 2);
    let source = l + r;
    let sink = l + r + 1;
    for (u, v) in edges {
        graph.add_edge(FlowEdge::new(u, l + v, 1));
    }
    for i in 0..l {
        graph.add_edge(FlowEdge::new(source, i, 1));
    }
    for i in 0..r {
        graph.add_edge(FlowEdge::new(l + i, sink, 1));
    }

    out.print_line(graph.fast_max_flow(source, sink));
    for i in 0..l {
        for e in &graph[i] {
            if e.capacity() == 0 && e.to() < l + r {
                out.print_line((i, e.to() - l));
                break;
            }
        }
    }
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
