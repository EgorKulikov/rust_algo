//{"name":"E. Innocent Students","group":"Codeforces - TheForces Round #35 (LOL-Forces)","url":"https://codeforces.com/gym/105390/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n3 5\n1 -2 3\n1 1 3 0\n2 3 -2\n1 2 3 1\n2 1 -4\n1 1 1 -10\n7 8\n1 2 4 2 4 6 7\n1 3 5 5\n1 1 7 5\n2 1 6\n2 4 4\n1 1 7 5\n1 1 3 2\n2 7 1\n1 1 7 -1\n","output":"1\n2\n1\n2\n3\n5\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EInnocentStudents"}}}

use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut a = input.read_int_vec(n);

    #[derive(Default)]
    struct Node {
        values: MultiTreeSet<i32>,
    }

    impl SegmentTreeNode for Node {}

    let mut st = SegmentTree::<Node>::new(n);
    for i in 0..n {
        st.point_through_update(i, |node| {
            node.values.insert(a[i]);
        });
    }

    for _ in 0..q {
        match input.read_int() {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let x = input.read_int();
                fn join_results(left_res: (i32, usize), right_res: (i32, usize)) -> (i32, usize) {
                    match left_res.0.cmp(&right_res.0) {
                        Ordering::Less => left_res,
                        Ordering::Equal => (left_res.0, left_res.1 + right_res.1),
                        Ordering::Greater => right_res,
                    }
                }
                let b = st
                    .for_each_with_init(
                        l..r,
                        |r, node| {
                            let left = node
                                .values
                                .range_rev(..&x)
                                .next()
                                .copied()
                                .map(|t| ((t - x).abs(), *node.values.get(&t).unwrap()))
                                .unwrap_or((i32::MAX, 0));
                            let right = node
                                .values
                                .range(&x..)
                                .next()
                                .copied()
                                .map(|t| ((t - x).abs(), *node.values.get(&t).unwrap()))
                                .unwrap_or((i32::MAX, 0));
                            let res = join_results(left, right);
                            join_results(r, res)
                        },
                        (i32::MAX, 0),
                    )
                    .1;
                out.print_line(b);
            }
            2 => {
                let at = input.read_size() - 1;
                let n_val = input.read_int();
                st.point_through_update(at, |node| {
                    node.values.remove(&a[at]);
                    node.values.insert(n_val);
                });
                a[at] = n_val;
            }
            _ => unreachable!(),
        }
    }
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

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./e_innocent_students";
        let time_limit = 4000;
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
fn e_innocent_students() {
    assert!(tester::run_tests());
}
