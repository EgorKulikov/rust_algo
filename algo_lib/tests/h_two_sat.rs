//{"name":"H - Two SAT","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_h","interactive":false,"timeLimit":5000,"tests":[{"input":"3 2\n1 4\n2 5\n0 6\n","output":"Yes\n4\n2\n0\n"},{"input":"3 3\n1 4\n2 5\n0 6\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HTwoSAT"}}}

use algo_lib::graph::two_sat::TwoSat;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let dd = input.read_int();
    let flags = input.read_int_pair_vec(n);

    let mut two_sat = TwoSat::new(n);
    for i in 0..n {
        let (a, b) = flags[i];
        for j in 0..i {
            let (c, d) = flags[j];
            if (a - c).abs() < dd {
                two_sat.add_or(i, true, j, true);
            }
            if (a - d).abs() < dd {
                two_sat.add_or(i, true, j, false);
            }
            if (b - c).abs() < dd {
                two_sat.add_or(i, false, j, true);
            }
            if (b - d).abs() < dd {
                two_sat.add_or(i, false, j, false);
            }
        }
    }
    out.set_bool_output(BoolOutput::YesNo);
    match two_sat.solve() {
        Some(result) => {
            out.print_line(true);
            for i in 0..n {
                if result[i] {
                    out.print_line(flags[i].0);
                } else {
                    out.print_line(flags[i].1);
                }
            }
        }
        None => {
            out.print_line(false);
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
    let path = "./h_two_sat";
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
fn h_two_sat() {
    assert!(tester::run_tests());
}
