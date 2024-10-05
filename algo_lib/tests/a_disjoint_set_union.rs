//{"name":"A - Disjoint Set Union","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_a","interactive":false,"timeLimit":5000,"tests":[{"input":"4 7\n1 0 1\n0 0 1\n0 2 3\n1 0 1\n1 1 2\n0 0 2\n1 1 3\n","output":"0\n1\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADisjointSetUnion"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    let mut dsu = DSU::new(n);
    for _ in 0..q {
        let t = input.read_int();
        let u = input.read_size();
        let v = input.read_size();
        match t {
            0 => {
                dsu.join(u, v);
            }
            1 => {
                out.print_line(if dsu.get(u) == dsu.get(v) { "1" } else { "0" });
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
    let path = "./a_disjoint_set_union";
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
fn a_disjoint_set_union() {
    assert!(tester::run_tests());
}
