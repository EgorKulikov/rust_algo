//{"name":"C. Currency","group":"Universal Cup - The 3rd Universal Cup. Stage 28: Haidian Huangzhuang","url":"https://contest.ucup.ac/contest/1903/problem/9692","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n2 3 5 2\n6 1 2 1 1\n1 2 4 2\n1 4 4\n2 3 1\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let h1 = input.read_long_vec(n - 1);
    let v = input.read_long_vec(n);
    let h2 = input.read_long_vec(n - 1);
    let constraints: Vec<(usize, usize, i64)> = input.read_vec(m).dec();

    let mut graph = Graph::new(n + 3);
    let source = n + 1;
    let sink = n + 2;
    let inf = i64::MAX as i128;
    graph.add_edge(FlowEdge::new(source, 0, 0));
    graph.add_edge(FlowEdge::new(0, sink, inf));
    graph.add_edge(FlowEdge::new(source, n, inf));
    graph.add_edge(FlowEdge::new(n, sink, 0));
    for i in 0..n {
        graph.add_edge(FlowEdge::new(i, i + 1, v[i] as i128));
        graph.add_edge(FlowEdge::new(i + 1, i, v[i] as i128));
    }
    for i in 0..n - 1 {
        graph.add_edge(FlowEdge::new(source, i + 1, h1[i] as i128));
        graph.add_edge(FlowEdge::new(i + 1, sink, h2[i] as i128));
    }
    for (i, j, c) in constraints {
        graph.add_edge(FlowEdge::new(j + 1, i + 1, c as i128));
    }
    // let level = Vec::with_gen(graph.vertex_count(), |i| {
    //     if i == source {
    //         0
    //     } else if i == sink {
    //         3
    //     } else if i % 2 == 0 {
    //         1
    //     } else {
    //         2
    //     }
    // });
    let ans = graph.max_flow(source, sink);
    // for i in 0..graph.vertex_count() {
    //     for e in &graph[i] {
    //         if e.capacity() == 0 && level[e.to()] == level[i] + 1 {
    //             eprintln!("{} -> {} {}", i, e.to(), e.flow(&graph));
    //         }
    //     }
    // }
    out.print_line(ans);
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
