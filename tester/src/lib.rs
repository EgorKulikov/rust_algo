use std::any::Any;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use crate::classic::run_single_test_classic;
use crate::interactive::run_single_test_interactive;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use test_set::{GeneratedTestSet, GeneratedTests, SampleTests, TestSet};

pub mod classic;
pub mod interactive;
mod print;
pub mod test_set;

pub enum Outcome {
    OK {
        duration: Duration,
        input_exhausted: bool,
    },
    TimeLimit {
        duration: Duration,
        input_exhausted: bool,
    },
    WrongAnswer {
        checker_output: String,
        input_exhausted: bool,
    },
    RuntimeError {
        panic_reason: String,
    },
}

pub enum Runner {
    Classic {
        checker: fn(Input, Option<Input>, Input) -> Result<(), String>,
    },
    Interactive {
        interactor: fn(Input, Output, Input) -> Result<(), String>,
    },
}

pub struct Tester {
    pub(crate) time_limit: u64,
    pub(crate) print_limit: usize,
    pub(crate) task_folder: String,
    pub(crate) solution: fn(Input, Output) -> bool,
    runner: Runner,
}

impl Tester {
    pub fn new_classic(
        time_limit: u64,
        print_limit: usize,
        task_folder: String,
        solution: fn(Input, Output) -> bool,
        checker: fn(Input, Option<Input>, Input) -> Result<(), String>,
    ) -> Self {
        Self {
            time_limit,
            print_limit,
            task_folder,
            solution,
            runner: Runner::Classic { checker },
        }
    }

    pub fn new_interactive(
        time_limit: u64,
        print_limit: usize,
        task_folder: String,
        solution: fn(Input, Output) -> bool,
        interactor: fn(Input, Output, Input) -> Result<(), String>,
    ) -> Self {
        Self {
            time_limit,
            print_limit,
            task_folder,
            solution,
            runner: Runner::Interactive { interactor },
        }
    }

    pub fn test_samples(&self) -> bool {
        let test_set = SampleTests {
            task_folder: self.task_folder.clone(),
        };
        self.test(test_set)
    }

    pub fn test_generated<Set: GeneratedTestSet>(
        &self,
        name: &str,
        print_details: bool,
        set: Set,
    ) -> bool {
        let test_set = GeneratedTests {
            name: name.to_string(),
            print_details,
            set,
        };
        self.test(test_set)
    }

    fn test<T: TestSet>(&self, test_set: T) -> bool {
        let mut test_failed = 0usize;
        let mut test_total = 0usize;
        print::start_test_set(test_set.name());
        for test in test_set.tests() {
            test_total += 1;
            let input = test_set.input(&test);
            let expected = test_set.output(&test, &input);
            print::start_test(
                &test,
                self.trim(&input),
                expected.as_ref().map(|v| self.trim(v.as_slice())),
                test_set.print_details(),
            );
            let outcome =
                self.run_single_test(&input, expected.as_deref(), test_set.print_details());
            if !matches!(outcome, Outcome::OK { .. }) {
                test_failed += 1;
                if !test_set.print_details() {
                    print::start_test(
                        &test,
                        self.trim(&input),
                        expected.as_ref().map(|v| self.trim(v.as_slice())),
                        true,
                    );
                    print::end_test(outcome, true);
                    File::create(format!("{}/tests/0.in", self.task_folder))
                        .unwrap()
                        .write_all(&input)
                        .unwrap();
                    if let Some(expected) = expected {
                        File::create(format!("{}/tests/0.out", self.task_folder))
                            .unwrap()
                            .write_all(&expected)
                            .unwrap();
                    }
                    return false;
                }
            }
            print::end_test(outcome, test_set.print_details());
        }
        print::end_test_set(test_failed, test_total);
        test_failed == 0
    }

    fn run_single_test(
        &self,
        input: &[u8],
        expected: Option<&[u8]>,
        print_details: bool,
    ) -> Outcome {
        match self.runner {
            Runner::Classic { checker } => {
                run_single_test_classic(self, checker, input, expected, print_details)
            }
            Runner::Interactive { interactor } => {
                run_single_test_interactive(self, interactor, input, expected, print_details)
            }
        }
    }

    fn trim<'a>(&self, s: &'a [u8]) -> &'a [u8] {
        if s.len() > self.print_limit {
            &s[..self.print_limit]
        } else {
            s
        }
    }
}

pub(crate) fn process_error(err: Box<dyn Any + Send>) -> Outcome {
    match err.downcast::<&str>() {
        Ok(as_string) => Outcome::RuntimeError {
            panic_reason: as_string.to_string(),
        },
        Err(_) => Outcome::RuntimeError {
            panic_reason: String::new(),
        },
    }
}
