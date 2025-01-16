//{"name":"L - Lazy Segment Tree","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_l","interactive":false,"timeLimit":5000,"tests":[{"input":"5 5\n0 1 0 0 1\n2 1 5\n1 3 4\n2 2 5\n1 1 3\n2 1 2\n","output":"2\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LLazySegmentTree"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    #[derive(Clone)]
    struct Node {
        zeroes: usize,
        ones: usize,
        num_inversions: usize,
        inv_num_inversions: usize,
        delta: bool,
    }

    impl Node {
        fn zero() -> Self {
            Self {
                zeroes: 1,
                ones: 0,
                num_inversions: 0,
                inv_num_inversions: 0,
                delta: false,
            }
        }

        fn one() -> Self {
            Self {
                zeroes: 0,
                ones: 1,
                num_inversions: 0,
                inv_num_inversions: 0,
                delta: false,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::zero()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.zeroes = left_val.zeroes + right_val.zeroes;
            self.ones = left_val.ones + right_val.ones;
            self.num_inversions = left_val.num_inversions
                + right_val.num_inversions
                + left_val.ones * right_val.zeroes;
            self.inv_num_inversions = left_val.inv_num_inversions
                + right_val.inv_num_inversions
                + left_val.zeroes * right_val.ones;
        }

        fn accumulate(&mut self, value: &Self) {
            self.push(&value.delta);
        }

        fn reset_delta(&mut self) {
            self.delta = false;
        }
    }

    impl Pushable<&bool> for Node {
        fn push(&mut self, delta: &bool) {
            if *delta {
                std::mem::swap(&mut self.zeroes, &mut self.ones);
                std::mem::swap(&mut self.num_inversions, &mut self.inv_num_inversions);
                self.delta = !self.delta;
            }
        }
    }

    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);

    let mut st = SegmentTree::with_gen(n, |i| if a[i] == 0 { Node::zero() } else { Node::one() });
    for _ in 0..q {
        let t = input.read_int();
        let l = input.read_size() - 1;
        let r = input.read_size();

        match t {
            1 => {
                st.update(l..r, &true);
            }
            2 => {
                out.print_line(st.query(l..r).num_inversions);
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

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./l_lazy_segment_tree";
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
fn l_lazy_segment_tree() {
    assert!(tester::run_tests());
}
