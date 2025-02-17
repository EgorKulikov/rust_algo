//{"name":"A. Assignment and Maximal Segment","group":"Codeforces - ITMO Academy: pilot course - Segment Tree, part 2 - Step 3","url":"https://codeforces.com/edu/course/2/lesson/5/3/practice/contest/280799/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5 3\n0 5 3\n1 3 -1\n3 4 -5\n","output":"15\n7\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    #[derive(Clone)]
    struct Node {
        ans: i64,
        prefix: i64,
        suffix: i64,
        sum: i64,
        delta: Option<i64>,
        len: i64,
    }

    impl Default for Node {
        fn default() -> Self {
            Self {
                ans: 0,
                prefix: 0,
                suffix: 0,
                sum: 0,
                delta: None,
                len: 1,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.ans = left_val
                .ans
                .max(right_val.ans)
                .max(left_val.suffix + right_val.prefix);
            self.prefix = left_val.prefix.max(left_val.sum + right_val.prefix);
            self.suffix = right_val.suffix.max(right_val.sum + left_val.suffix);
            self.sum = left_val.sum + right_val.sum;
            self.len = left_val.len + right_val.len;
        }

        fn accumulate(&mut self, delta: &Self) {
            if let Some(delta) = delta.delta {
                let x = delta.max(0);
                self.ans = x * self.len;
                self.prefix = x * self.len;
                self.suffix = x * self.len;
                self.sum = delta * self.len;
                self.delta = Some(delta);
            }
        }

        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }

    let mut st = SegmentTree::<Node>::new(n);

    for _ in 0..m {
        let l = input.read_size();
        let r = input.read_size();
        let v = input.read_long();
        st.update(
            l..r,
            &Node {
                delta: Some(v),
                ..Default::default()
            },
        );
        out.print_line(st.query(..).ans);
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
        let path = "./a_assignment_and_maximal_segment";
        let tl = 1000;
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
fn a_assignment_and_maximal_segment() {
    assert!(tester::run_tests());
}
