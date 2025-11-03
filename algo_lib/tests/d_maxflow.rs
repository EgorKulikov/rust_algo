//{"name":"D - Maxflow","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_d","interactive":false,"timeLimit":5000,"tests":[{"input":"3 3\n#..\n..#\n...\n","output":"3\n#><\nvv#\n^^.\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaxflow"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::{LegacyTaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut s = input.read_char_table(n, m);

    let mut graph = Graph::new(n * m + 2);
    let source = n * m;
    let sink = n * m + 1;
    for i in 0..n {
        for j in 0..m {
            if s[(i, j)] == b'#' {
                continue;
            }
            if (i + j) % 2 == 0 {
                graph.add_edge(FlowEdge::new(source, i * m + j, 1));
                for (r, c) in D4::iter(i, j, n, m) {
                    if s[(r, c)] == b'.' {
                        graph.add_edge(FlowEdge::new(i * m + j, r * m + c, 1));
                    }
                }
            } else {
                graph.add_edge(FlowEdge::new(i * m + j, sink, 1));
            }
        }
    }
    out.print_line(graph.max_flow(source, sink));
    for i in 0..n {
        for j in 0..m {
            if (i + j) % 2 == 0 {
                for e in &graph[i * m + j] {
                    if e.to() < source && e.flow(&graph) == 1 {
                        let (r, c) = (e.to() / m, e.to() % m);
                        if r == i {
                            if c == j + 1 {
                                s[(i, j)] = b'>';
                                s[(r, c)] = b'<';
                            } else if c == j - 1 {
                                s[(i, j)] = b'<';
                                s[(r, c)] = b'>';
                            }
                        } else if c == j {
                            if r == i + 1 {
                                s[(i, j)] = b'v';
                                s[(r, c)] = b'^';
                            } else if r == i - 1 {
                                s[(i, j)] = b'^';
                                s[(r, c)] = b'v';
                            }
                        }
                    }
                }
            }
        }
    }
    out.print_table(&s);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: LegacyTaskType = LegacyTaskType::Classic;

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
        LegacyTaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        LegacyTaskType::Interactive => true,
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
        let path = "./d_maxflow";
        let time_limit = 5000;
        let tester = match TASK_TYPE {
            crate::LegacyTaskType::Interactive => {
                Tester::new_interactive(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    std_interactor,
                )
                //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::LegacyTaskType::Classic => {
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
fn d_maxflow() {
    assert!(tester::run_tests());
}
