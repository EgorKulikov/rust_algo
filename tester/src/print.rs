use crate::test_set::TestSet;
use crate::Outcome;
use std::fmt::Display;
use std::time::Duration;

pub(crate) const BLUE: &str = "\x1B[36m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
pub(crate) const DEF: &str = "\x1B[0m";

pub(crate) fn start_test_set(name: &str) {
    println!("=====================================================");
    println!("{}Test set: {}{}", BLUE, name, DEF);
}

fn print_score_summary(scores: &[i64]) {
    if scores.is_empty() {
        return;
    }
    let count = scores.len();
    let sum: i64 = scores.iter().sum();
    let mean = sum as f64 / count as f64;
    let mut sorted: Vec<i64> = scores.to_vec();
    sorted.sort_unstable();
    let median = sorted[(count - 1) / 2];
    let min = *sorted.first().unwrap();
    let max = *sorted.last().unwrap();
    println!(
        "{}=== Scores: count={} sum={} mean={:.2} median={} min={} max={} ==={}",
        BLUE, count, sum, mean, median, min, max, DEF
    );
}

pub(crate) fn end_test_set(
    test_failed: usize,
    test_total: usize,
    max_time: Duration,
    scores: &[Option<i64>],
) {
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed, max time elapsed: {:.3}s{}",
            BLUE,
            GREEN,
            test_total,
            BLUE,
            max_time.as_secs_f64(),
            DEF
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            RED, test_failed, test_total, BLUE, DEF
        );
    }
    let scored: Vec<i64> = scores.iter().filter_map(|s| *s).collect();
    print_score_summary(&scored);
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
        println!("{}Output: {}", BLUE, DEF);
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
            score,
        } => {
            let score_suffix = match score {
                Some(s) => format!(" [score={}]", s),
                None => String::new(),
            };
            if print_details {
                print!(
                    "{}Time elapsed: {:.3}s",
                    BLUE,
                    (duration.as_millis() as f64) / 1000.,
                );
                println!("{}", DEF);
                if !input_exhausted {
                    println!("{}Input not exhausted{}", RED, DEF);
                }
                println!("{}Verdict: {}OK{}{}", BLUE, GREEN, score_suffix, DEF);
            } else {
                println!("{}OK{}{}", GREEN, score_suffix, DEF);
            }
        }
        Outcome::TimeLimit {
            duration,
            input_exhausted,
        } => {
            if print_details {
                print!(
                    "{}Time elapsed: {:.3}s",
                    BLUE,
                    (duration.as_millis() as f64) / 1000.,
                );
                println!("{}", DEF);
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
