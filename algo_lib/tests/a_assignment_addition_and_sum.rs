//{"name":"A. Assignment, Addition, and Sum","group":"Codeforces - ITMO Academy: pilot course - Segment Tree, part 2 - Step 4","url":"https://codeforces.com/edu/course/2/lesson/5/4/practice/contest/280801/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5 7\n1 0 3 3\n2 2 4 2\n3 1 3\n2 1 5 1\n1 0 2 2\n3 0 3\n3 3 5\n","output":"8\n10\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
        val: i64,
        delta: Option<i64>,
        add: i64,
        len: i64,
    }

    impl Default for Node {
        fn default() -> Self {
            Self {
                val: 0,
                delta: None,
                add: 0,
                len: 1,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val + right_val.val;
            self.len = left_val.len + right_val.len;
        }

        fn accumulate(&mut self, delta: &Self) {
            if let Some(delta) = delta.delta {
                self.val = delta * self.len;
                self.delta = Some(delta);
                self.add = 0;
            }
            self.val += delta.add * self.len;
            self.add += delta.add;
        }

        fn reset_delta(&mut self) {
            self.delta = None;
            self.add = 0;
        }
    }

    let mut st = SegmentTree::<Node>::new(n);

    for _ in 0..m {
        let t = input.read_int();
        let l = input.read_size();
        let r = input.read_size();
        match t {
            1 => {
                let v = input.read_long();
                st.update(
                    l..r,
                    &Node {
                        delta: Some(v),
                        ..Default::default()
                    },
                );
            }
            2 => {
                let v = input.read_long();
                st.update(
                    l..r,
                    &Node {
                        add: v,
                        ..Default::default()
                    },
                );
            }
            3 => {
                out.print_line(st.query(l..r).val);
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

fn interact(mut sol_input: Input, mut sol_output: Output, mut input: Input) -> Result<(), String> {
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

    fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
        Box::new(1..=1)
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new();
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./a_assignment_addition_and_sum";
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
fn a_assignment_addition_and_sum() {
    assert!(tester::run_tests());
}
