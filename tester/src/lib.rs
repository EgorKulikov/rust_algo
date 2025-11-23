use std::any::Any;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use crate::classic::run_single_test_classic;
use crate::interactive::{run_single_test_interactive, SolutionRunner};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
#[cfg(not(feature = "test"))]
use test_set::GeneratedTests;
use test_set::{GeneratedTestSet, SampleTests, TestSet};

pub mod classic;
pub mod interactive;
mod print;
pub mod test_set;

pub enum Outcome {
    OK {
        duration: Duration,
        second_duration: Option<Duration>,
        input_exhausted: bool,
    },
    TimeLimit {
        duration: Duration,
        second_duration: Option<Duration>,
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
        interactor: fn(Input, Option<Input>, SolutionRunner) -> Result<(), String>,
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
        interactor: fn(Input, Option<Input>, SolutionRunner) -> Result<(), String>,
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

    #[cfg(feature = "test")]
    pub fn test_generated<Set: GeneratedTestSet>(
        &self,
        _name: &str,
        _print_details: bool,
        _set: Set,
    ) -> bool {
        true
    }

    #[cfg(not(feature = "test"))]
    pub fn test_generated<Set: GeneratedTestSet>(
        &self,
        name: &str,
        print_details: bool,
        set: Set,
    ) -> bool
    where
        Set::TestId: 'static,
    {
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
        let mut max_time = Duration::default();
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
            let outcome = self.run_single_test(&input, expected.as_deref(), &test_set, &test);
            if let Outcome::OK {
                duration,
                second_duration,
                ..
            } = outcome
            {
                max_time = max_time.max(duration);
                if let Some(sd) = second_duration {
                    max_time = max_time.max(sd);
                }
            } else {
                test_failed += 1;
                if !test_set.print_details() {
                    print::start_test(
                        &test,
                        self.trim(&input),
                        expected.as_ref().map(|v| self.trim(v.as_slice())),
                        true,
                    );
                    print::end_test(outcome, true);
                    for i in (0..1000).rev() {
                        let in_file =
                            format!("tasks/{}/tests/.failed_{:03}.in", self.task_folder, i);
                        if Path::new(&in_file).exists() {
                            continue;
                        }
                        File::create(in_file).unwrap().write_all(&input).unwrap();
                        if let Some(expected) = expected {
                            File::create(format!(
                                "tasks/{}/tests/.failed_{:03}.ans",
                                self.task_folder, i
                            ))
                            .unwrap()
                            .write_all(&expected)
                            .unwrap();
                        }
                        break;
                    }
                    return false;
                }
            }
            print::end_test(outcome, test_set.print_details());
        }
        print::end_test_set(test_failed, test_total, max_time);
        test_failed == 0
    }

    fn run_single_test<T: TestSet>(
        &self,
        input: &[u8],
        expected: Option<&[u8]>,
        test_set: &T,
        test_id: &T::TestId,
    ) -> Outcome {
        match self.runner {
            Runner::Classic { checker } => {
                run_single_test_classic(self, checker, input, expected, test_set, test_id)
            }
            Runner::Interactive { interactor } => run_single_test_interactive(
                self,
                interactor,
                input,
                expected,
                test_set.print_details(),
            ),
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
