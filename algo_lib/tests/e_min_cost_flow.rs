//{"name":"E - MinCostFlow","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_e","interactive":false,"timeLimit":5000,"tests":[{"input":"3 1\n5 3 2\n1 4 8\n7 6 9\n","output":"19\nX..\n..X\n.X.\n"},{"input":"3 2\n10 10 1\n10 10 1\n1 1 10\n","output":"50\nXX.\nXX.\n..X\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMinCostFlow"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::min_cost_flow::MinCostFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_table(n, n);

    let mut graph = Graph::new(2 * n + 2);
    let source = 2 * n;
    let sink = 2 * n + 1;
    for i in 0..n {
        graph.add_edge(WeightedFlowEdge::new(source, i, 0, k));
        graph.add_edge(WeightedFlowEdge::new(i + n, sink, 0, k));
        for j in 0..n {
            graph.add_edge(WeightedFlowEdge::new(i, j + n, -a[i][j], 1));
        }
    }
    let (cost, _) = graph.min_cost_flow(source, sink);
    out.print_line(-cost);
    let mut ans = Arr2d::new(n, n, b'.');
    for i in 0..n {
        for e in &graph[i] {
            if e.to() < source && e.flow(&graph) == 1 {
                ans[i][e.to() - n] = b'X';
            }
        }
    }
    out.print_table(&ans);
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]

    use crate::{run, TASK_TYPE};
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use tester::classic::default_checker;
    use tester::interactive::std_interactor;
    use tester::test_set::GeneratedTestSet;
    use tester::Tester;

    const PRINT_LIMIT: usize = 1000;

    fn interact(
        mut sol_input: Input,
        mut sol_output: Output,
        mut input: Input,
    ) -> Result<(), String> {
        Ok(())
    }

    fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
        Ok(())
    }

    struct StressTest;

    impl GeneratedTestSet for StressTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./e_min_cost_flow";
        let time_limit = 5000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    std_interactor,
                )
                //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    default_checker,
                )
                //Tester::new_classic(time_limit, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn e_min_cost_flow() {
    assert!(tester::run_tests());
}
