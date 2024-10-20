//{"name":"O. Sub Brackets","group":"Universal Cup - The 3rd Universal Cup. Stage 13: Sendai","url":"https://contest.ucup.ac/contest/1812/problem/9490","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n1 2\n4 5\n2 5\n","output":"2\n"},{"input":"2 4\n1 2\n1 2\n1 2\n1 2\n","output":"4\n"},{"input":"32 11\n25 32\n19 32\n11 24\n20 31\n22 25\n21 26\n17 22\n30 31\n23 28\n4 15\n19 22\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"OSubBrackets"}}}

use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let m = input.read_size();
    let segments = input.read_size_pair_vec(m);

    let mut graph = Graph::new(m + 2).do_with(|graph| {
        for i in 0..m {
            if segments[i].0 % 2 == 0 {
                graph.add_edge(FlowEdge::new(m, i, 1));
                for j in 0..m {
                    if segments[j].0 % 2 == 1
                        && (segments[i].0 <= segments[j].0
                            && segments[j].0 <= segments[i].1
                            && segments[i].1 <= segments[j].1
                            || segments[j].0 <= segments[i].0
                                && segments[i].0 <= segments[j].1
                                && segments[j].1 <= segments[i].1)
                    {
                        graph.add_edge(FlowEdge::new(i, j, 1));
                    }
                }
            } else {
                graph.add_edge(FlowEdge::new(i, m + 1, 1));
            }
        }
    });
    out.print_line(m - graph.max_flow(m, m + 1));
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
