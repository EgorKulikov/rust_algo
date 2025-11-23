//{"name":"C. Painter","group":"Codeforces - ITMO Academy: pilot course - Segment Tree, part 2 - Step 4","url":"https://codeforces.com/edu/course/2/lesson/5/4/practice/contest/280801/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\nW 2 3\nB 2 2\nB 4 2\nB 3 2\nB 7 2\nW 3 1\nW 0 10\n","output":"0 0\n1 2\n1 4\n1 4\n2 6\n3 5\n0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::LegacyTestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    #[derive(Clone)]
    struct Node {
        black: usize,
        internal: usize,
        total: usize,
        left: bool,
        right: bool,
        all: bool,
        delta: Option<bool>,
        len: usize,
    }

    impl Default for Node {
        fn default() -> Self {
            Self {
                black: 0,
                internal: 0,
                total: 0,
                left: false,
                right: false,
                all: false,
                delta: None,
                len: 1,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.black = left_val.black + right_val.black;
            self.internal = left_val.internal
                + right_val.internal
                + if !left_val.all && !right_val.all && (left_val.right || right_val.left) {
                    1
                } else {
                    0
                };
            self.left = left_val.left || left_val.all && !right_val.all;
            self.right = right_val.right || right_val.all && !left_val.all;
            self.all = left_val.all && right_val.all;
            self.total = self.internal
                + if self.all {
                    1
                } else {
                    self.left as usize + self.right as usize
                };
            self.len = left_val.len + right_val.len;
        }

        fn accumulate(&mut self, delta: &Self) {
            if let Some(delta) = delta.delta {
                if delta {
                    self.black = self.len;
                    self.internal = 0;
                    self.total = 1;
                    self.left = false;
                    self.right = false;
                    self.all = true;
                    self.delta = Some(true);
                } else {
                    self.black = 0;
                    self.internal = 0;
                    self.total = 0;
                    self.left = false;
                    self.right = false;
                    self.all = false;
                    self.delta = Some(false);
                }
            }
        }

        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }

    let mut st = SegmentTree::<Node>::new(1_000_001);

    for _ in 0..n {
        let c = input.read_char();
        let x = (input.read_int() + 500_000) as usize;
        let l = input.read_size();
        st.update(
            x..x + l,
            &Node {
                delta: Some(c == b'B'),
                ..Default::default()
            },
        );
        let res = st.query(..);
        out.print_line((res.total, res.black));
    }
}

pub static TEST_TYPE: LegacyTestType = LegacyTestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        LegacyTestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        LegacyTestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        LegacyTestType::MultiEof => {
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
        let path = "./c_painter";
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
fn c_painter() {
    assert!(tester::run_tests());
}
