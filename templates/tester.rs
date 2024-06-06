#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crate::run;
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
    let path = "./$TASK";
    let time_limit = $TIME_LIMIT;
    let tester = if $INTERACTIVE {
        Tester::new_interactive(
            time_limit,
            PRINT_LIMIT,
            path.to_string(),
            run,
            std_interactor,
        )
        //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
    } else {
        Tester::new_classic(
            time_limit,
            PRINT_LIMIT,
            path.to_string(),
            run,
            default_checker,
        )
        //Tester::new_classic(time_limit, PRINT_LIMIT, path.to_string(), run, check)
    };
    let passed = tester.test_samples();
    // tester.test_generated("Stress test", false, StressTest);
    passed
}
