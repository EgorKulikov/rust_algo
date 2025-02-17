//{"name":"B. Add Arithmetic Progression On Segment","group":"Codeforces - ITMO Academy: pilot course - Segment Tree, part 2 - Step 4","url":"https://codeforces.com/edu/course/2/lesson/5/4/practice/contest/280801/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"9 10\n1 4 8 1 2\n2 6\n2 8\n1 1 9 3 1\n2 6\n2 8\n1 2 5 2 3\n2 1\n2 2\n2 3\n","output":"5\n9\n13\n19\n3\n6\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    #[derive(Clone, Default)]
    struct Node {
        left: i64,
        sum_a: i64,
        sum_d: i64,
    }

    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, delta: &Self) {
            self.sum_a += delta.sum_a + (self.left - delta.left) * delta.sum_d;
            self.sum_d += delta.sum_d;
        }

        fn reset_delta(&mut self) {
            self.sum_a = 0;
            self.sum_d = 0;
        }
    }

    let mut st = SegmentTree::with_gen_full(n, |l, _| Node {
        left: l as i64,
        ..Default::default()
    });

    for _ in 0..m {
        let t = input.read_int();
        match t {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let a = input.read_long();
                let d = input.read_long();
                st.update(
                    l..r,
                    &Node {
                        sum_a: a,
                        sum_d: d,
                        left: l as i64,
                    },
                );
            }
            2 => {
                let pos = input.read_size() - 1;
                out.print_line(st.point_query(pos).sum_a);
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]
    #![allow(unused_imports)]

    use crate::{run, TASK_TYPE};
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::random::Random;
    use tester::classic::default_checker;
    use tester::classic::EPS;
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

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let mut r = Random::new();
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..=1
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let mut r = Random::new();
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./b_add_arithmetic_progression_on_segment";
        let tl = 2000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
                // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn b_add_arithmetic_progression_on_segment() {
    assert!(tester::run_tests());
}
