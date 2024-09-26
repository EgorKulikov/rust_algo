//{"name":"B. Binary Search Tree","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"insert 2\ninsert 5\ninsert 3\nexists 2\nexists 4\nnext 4\nprev 4\ndelete 5\nnext 4\nprev 4\n","output":"true\nfalse\n5\n3\nnone\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBinarySearchTree"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut set = TreapSet::new();

    while !input.is_empty() {
        let command = input.read_str();
        let value = input.read_int();
        match command.as_ref() {
            b"insert" => {
                set.insert(value);
            }
            b"delete" => {
                set.remove(&value);
            }
            b"exists" => {
                if set.contains(&value) {
                    out.print_line("true");
                } else {
                    out.print_line("false");
                }
            }
            b"next" => {
                if let Some(next) = set.higher(&value) {
                    out.print_line(next);
                } else {
                    out.print_line("none");
                }
            }
            b"prev" => {
                if let Some(prev) = set.lower(&value) {
                    out.print_line(prev);
                } else {
                    out.print_line("none");
                }
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

fn interact(mut sol_input: Input, mut sol_output: Output, mut input: Input) -> Result<(), String> {
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
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./b_binary_search_tree";
    let time_limit = 3000;
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
fn b_binary_search_tree() {
    assert!(tester::run_tests());
}
