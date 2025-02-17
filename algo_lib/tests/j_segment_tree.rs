//{"name":"J - Segment Tree","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_j","interactive":false,"timeLimit":5000,"tests":[{"input":"5 5\n1 2 3 2 1\n2 1 5\n3 2 3\n1 3 1\n2 2 4\n3 1 3\n","output":"3\n3\n2\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JSegmentTree"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::extensions::option::OptionExt;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    #[derive(Copy, Clone, Default)]
    struct Node {
        max: i32,
        left: usize,
        right: usize,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.left = left_val.left;
            self.right = right_val.right;
            self.max = left_val.max.max(right_val.max);
        }

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n);

    let mut st = SegmentTree::with_gen(n, |i| Node {
        max: a[i],
        left: i,
        right: i + 1,
    });
    for _ in 0..q {
        match input.read_int() {
            1 => {
                let x = input.read_size() - 1;
                let v = input.read_int();
                st.point_update(
                    x,
                    Node {
                        max: v,
                        left: x,
                        right: x + 1,
                    },
                );
            }
            2 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                out.print_line(st.query(l..r).max);
            }
            3 => {
                let x = input.read_size() - 1;
                let v = input.read_int();
                let base_node = st.for_each(x.., |res: Option<Node>, node| {
                    if res.is_none() {
                        (*node).take_if(node.max >= v)
                    } else {
                        res
                    }
                });
                let ans = if let Some(base_node) = base_node {
                    st.binary_search(
                        |left, _| {
                            if left.right <= base_node.left || left.max < v {
                                Direction::Right
                            } else {
                                Direction::Left
                            }
                        },
                        |_, at| at,
                    )
                } else {
                    n
                };
                out.print_line(ans + 1);
            }
            _ => unreachable!(),
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
        let path = "./j_segment_tree";
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
fn j_segment_tree() {
    assert!(tester::run_tests());
}
