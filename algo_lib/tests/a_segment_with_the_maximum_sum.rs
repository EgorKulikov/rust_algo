//{"name":"A. Segment with the Maximum Sum","group":"Codeforces - ITMO Academy: pilot course - Segment Tree, part 1 - Step 2","url":"https://codeforces.com/edu/course/2/lesson/4/2/practice/contest/273278/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n5 -4 4 3 -5\n4 3\n3 -1\n","output":"8\n11\n7\n"},{"input":"4 2\n-2 -1 -5 -4\n1 3\n3 2\n","output":"0\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::LegacyTaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        sum: i64,
        prefix: i64,
        suffix: i64,
        ans: i64,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.sum = left_val.sum + right_val.sum;
            self.prefix = left_val.prefix.max(left_val.sum + right_val.prefix);
            self.suffix = right_val.suffix.max(right_val.sum + left_val.suffix);
            self.ans = left_val
                .ans
                .max(right_val.ans)
                .max(left_val.suffix + right_val.prefix);
        }
    }

    impl Node {
        fn new(val: i64) -> Self {
            Self {
                sum: val,
                prefix: val.max(0),
                suffix: val.max(0),
                ans: val.max(0),
            }
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node::new(a[i]));
    out.print_line(st.query(..).ans);
    for _ in 0..m {
        let i = input.read_size();
        let v = input.read_long();
        st.point_update(i, Node::new(v));
        out.print_line(st.query(..).ans);
    }
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
        LegacyTaskType::Classic => input.is_empty(),
        LegacyTaskType::Interactive => true,
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
        let path = "./a_segment_with_the_maximum_sum";
        let tl = 1000;
        let tester = match TASK_TYPE {
            crate::LegacyTaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::LegacyTaskType::Classic => {
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
fn a_segment_with_the_maximum_sum() {
    assert!(tester::run_tests());
}
