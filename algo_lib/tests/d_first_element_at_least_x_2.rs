//{"name":"D. First element at least X - 2","group":"Codeforces - ITMO Academy: pilot course - Segment Tree, part 1 - Step 2","url":"https://codeforces.com/edu/course/2/lesson/4/2/practice/contest/273278/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5 7\n1 3 2 4 3\n2 3 0\n2 3 2\n1 2 5\n2 4 1\n2 5 4\n1 3 7\n2 6 1\n","output":"1\n3\n2\n-1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::direction::Direction;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);

    #[derive(Default, Clone)]
    struct Node(i32);

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.0 = left_val.0.max(right_val.0);
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node(a[i]));
    for _ in 0..m {
        let t = input.read_int();
        match t {
            1 => {
                let i = input.read_size();
                let v = input.read_int();
                st.point_update(i, Node(v));
            }
            2 => {
                let x = input.read_int();
                let l = input.read_size();
                out.print_line(st.binary_search_in(
                    l..,
                    |node| node.0 >= x,
                    |left, _| {
                        if left.0 >= x {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    },
                    |_, at| at,
                ));
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
    let path = "./d_first_element_at_least_x_2";
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
fn d_first_element_at_least_x_2() {
    assert!(tester::run_tests());
}
