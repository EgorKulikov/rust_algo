use crate::test_set::TestSet;
use crate::Outcome;
use std::fmt::Display;

const BLUE: &str = "\x1B[36m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const DEF: &str = "\x1B[0m";

pub(crate) fn start_test_set(name: &str) {
    println!("=====================================================");
    println!("{}Test set: {}{}", BLUE, name, DEF);
}

pub(crate) fn end_test_set(test_failed: usize, test_total: usize) {
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed{}",
            BLUE, GREEN, test_total, BLUE, DEF
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            RED, test_failed, test_total, BLUE, DEF
        );
    }
}

pub(crate) fn start_test<TestId: Display>(
    name: &TestId,
    input: &[u8],
    expected: Option<&[u8]>,
    print_details: bool,
) {
    if print_details {
        println!("=====================================================");
        println!("{}Test {}{}", BLUE, name, DEF);
        println!("{}Input:{}", BLUE, DEF);
        println!("{}", String::from_utf8_lossy(input));
        if let Some(expected) = expected {
            println!("{}Expected:{}", BLUE, DEF);
            println!("{}", String::from_utf8_lossy(expected));
        }
    } else {
        print!("{}Test {}{} ... ", BLUE, name, DEF);
    }
}

pub(crate) fn print_output(output: &[u8], print_details: bool) {
    if print_details && !output.is_empty() {
        println!("{}Output:{}", BLUE, DEF);
        println!("{}", String::from_utf8_lossy(output));
    }
}

pub(crate) fn print_diff<T: TestSet>(test_set: &T, test_id: &T::TestId) {
    test_set.output_diff(test_id);
}

pub(crate) fn print_interacting(print_details: bool) {
    if print_details {
        println!("{}Interacting{}", BLUE, DEF);
    }
}

pub(crate) fn end_test(outcome: Outcome, print_details: bool) {
    match outcome {
        Outcome::OK {
            duration,
            input_exhausted,
        } => {
            if print_details {
                println!(
                    "{}Time elapsed: {:.3}s{}",
                    BLUE,
                    (duration.as_millis() as f64) / 1000.,
                    DEF,
                );
                if !input_exhausted {
                    println!("{}Input not exhausted{}", RED, DEF);
                }
                println!("{}Verdict: {}OK{}", BLUE, GREEN, DEF);
            } else {
                println!("{}OK{}", GREEN, DEF);
            }
        }
        Outcome::TimeLimit {
            duration,
            input_exhausted,
        } => {
            if print_details {
                println!(
                    "{}Time elapsed: {:.3}s{}",
                    BLUE,
                    (duration.as_millis() as f64) / 1000.,
                    DEF,
                );
                if !input_exhausted {
                    println!("{}Input not exhausted{}", RED, DEF);
                }
                println!("{}Verdict: {}Time Limit{}", BLUE, RED, DEF);
            } else {
                println!("{}Time Limit{}", RED, DEF);
            }
        }
        Outcome::WrongAnswer {
            checker_output,
            input_exhausted,
        } => {
            if print_details {
                if !input_exhausted {
                    println!("{}Input not exhausted{}", RED, DEF);
                }
                println!(
                    "{}Verdict: {}Wrong Answer ({}){}",
                    BLUE, RED, checker_output, DEF
                );
            } else {
                println!("{}Wrong Answer{}", RED, DEF);
            }
        }
        Outcome::RuntimeError { panic_reason } => {
            if print_details {
                println!(
                    "{}Verdict: {}RuntimeError ({:?}){}",
                    BLUE, RED, panic_reason, DEF
                );
            } else {
                println!("{}RuntimeError{}", RED, DEF);
            }
        }
    }
}
