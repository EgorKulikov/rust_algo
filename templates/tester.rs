#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::{run, TASK_TYPE, TEST_TYPE, TestType};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::Random;
use algo_lib::string::str::StrReader;
use std::io::{Read, Write};
use std::thread::yield_now;
use tester::classic::default_checker;
use tester::classic::EPS;
use tester::interactive::std_interactor;
use tester::interactive::SolutionRunner;
use tester::test_set::GeneratedTestSet;
use tester::Tester;

const PRINT_LIMIT: usize = 1000;

fn interact(mut input: Input, expected: Option<Input>, mut runner: SolutionRunner) -> Result<(), String> {
    let (mut sol, mut out) = runner.run();
    Ok(())
}

fn run_twice(
    mut input: Input,
    expected: Option<Input>,
    mut runner: SolutionRunner,
) -> Result<(), String> {
    let (mut sol, mut out) = runner.run();
    input.read_line();
    out.print_line("first");
    let t = match TEST_TYPE {
        TestType::RunTwiceSingle => None,
        TestType::RunTwiceMultiNumber => {
            let t = input.read_size();
            out.print_line(t);
            Some(t)
        }
        _ => unreachable!(),
    };
    let mut input_vec = Vec::new();
    input.read_to_end(&mut input_vec).unwrap();
    out.write_all(&input_vec).unwrap();
    out.flush();
    while !runner.is_finished() {
        yield_now();
    }
    let mut first_output = Vec::new();
    sol.read_to_end(&mut first_output).unwrap();

    let (mut sol, mut out) = runner.run();
    out.print_line("second");
    if let Some(t) = t {
        out.print_line(t);
    }
    out.write_all(&first_output).unwrap();
    out.flush();
    let mut ans = Vec::new();
    sol.read_to_end(&mut ans).unwrap();
    default_checker(Input::slice(&input_vec), expected, Input::slice(&ans))
    // check(Input::slice(&input_vec), expected, Input::slice(&ans))
}

fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
    Ok(())
}

struct StressTest;

impl GeneratedTestSet for StressTest {
    type TestId = usize;

    fn tests(&self) -> impl Iterator<Item=Self::TestId> {
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

    fn tests(&self) -> impl Iterator<Item=Self::TestId> {
        1..=1
    }

    fn input(&self, test: &Self::TestId, out: &mut Output) {
        let mut r = Random::new_with_seed(239);
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./$TASK";
    let tl = $TIME_LIMIT;
    let tester = match TASK_TYPE {
        crate::TaskType::Interactive => {
            Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, run_twice)
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
