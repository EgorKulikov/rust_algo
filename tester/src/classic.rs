use crate::print::{print_diff, print_output};
use crate::test_set::TestSet;
use crate::{process_error, Outcome, Tester};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cell::Cell;

pub(crate) fn run_single_test_classic<T: TestSet>(
    tester: &Tester,
    checker: fn(Input, Option<Input>, Input) -> Result<(), String>,
    input: &[u8],
    expected: Option<&[u8]>,
    test_set: &T,
    test_id: &T::TestId,
) -> Outcome {
    let input = &input;
    let (outcome, output) = match std::panic::catch_unwind(|| {
        let input = *input;
        let started = std::time::Instant::now();
        let mut output = Vec::new();
        let is_exhausted = (tester.solution)(Input::slice(input), Output::buf(&mut output));
        let res = started.elapsed();
        (output, res, is_exhausted)
    }) {
        Ok((output, duration, is_exhausted)) => {
            test_set.save_output(test_id, &output);
            let input = *input;
            let checker_result = (checker)(
                Input::slice(input),
                expected.map(Input::slice),
                Input::slice(output.as_slice()),
            );
            if let Err(checker_output) = checker_result {
                (
                    Outcome::WrongAnswer {
                        input_exhausted: is_exhausted,
                        checker_output,
                    },
                    output,
                )
            } else if duration.as_millis() as u64 > tester.time_limit {
                (
                    Outcome::TimeLimit {
                        duration,
                        input_exhausted: is_exhausted,
                    },
                    output,
                )
            } else {
                (
                    Outcome::OK {
                        duration,
                        input_exhausted: is_exhausted,
                    },
                    output,
                )
            }
        }
        Err(err) => (process_error(err), Vec::new()),
    };
    if test_set.print_details() || !matches!(outcome, Outcome::OK { .. }) {
        print_output(tester.trim(&output), true);
    }
    if test_set.print_details() && matches!(outcome, Outcome::WrongAnswer { .. }) {
        print_diff(test_set, test_id);
    }
    outcome
}

pub fn default_checker(
    _input: Input,
    expected: Option<Input>,
    mut actual: Input,
) -> Result<(), String> {
    if expected.is_none() {
        return Ok(());
    }
    let mut expected = expected.unwrap();
    let mut token_num = 0;
    loop {
        let expected_token = expected.next_token();
        let actual_token = actual.next_token();
        if expected_token != actual_token {
            if expected_token.is_none() {
                return Err(format!("Expected has only {} tokens", token_num));
            } else if actual_token.is_none() {
                return Err(format!("Actual has only {} tokens", token_num));
            } else {
                return Err(format!(
                    "Token #{} differs, expected {}, actual {}",
                    token_num,
                    String::from_utf8(expected_token.unwrap()).unwrap(),
                    String::from_utf8(actual_token.unwrap()).unwrap()
                ));
            }
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

thread_local! {
    pub static EPS: Cell<f64> = const { Cell::new(1e-9) };
}

fn default_checker_eps(
    expected: Option<Input>,
    mut actual: Input,
    check_real: impl Fn(f64, f64) -> bool,
) -> Result<(), String> {
    if expected.is_none() {
        return Ok(());
    }
    let mut expected = expected.unwrap();
    let mut token_num = 0;
    loop {
        let expected_token = expected.next_token();
        let actual_token = actual.next_token();
        if expected_token != actual_token {
            match (expected_token.as_ref(), actual_token.as_ref()) {
                (None, _) => return Err(format!("Expected has only {} tokens", token_num)),
                (_, None) => return Err(format!("Actual has only {} tokens", token_num)),
                (Some(expected_token), Some(actual_token)) => {
                    let expected_token = String::from_utf8(expected_token.clone()).unwrap();
                    let actual_token = String::from_utf8(actual_token.clone()).unwrap();
                    let mut failed = true;
                    if let Ok(expected) = expected_token.parse::<f64>() {
                        if let Ok(actual) = actual_token.parse::<f64>() {
                            if check_real(expected, actual) {
                                failed = false;
                            }
                        }
                    }
                    if failed {
                        return Err(format!(
                            "Token #{} differs, expected {}, actual {}",
                            token_num, expected_token, actual_token,
                        ));
                    }
                }
            }
        }
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

pub fn default_checker_eps_rel(
    _input: Input,
    expected: Option<Input>,
    actual: Input,
) -> Result<(), String> {
    default_checker_eps(expected, actual, |expected, actual| {
        let by = expected.abs().max(1.);
        (expected - actual).abs() < EPS.with(|eps| eps.get()) * by
    })
}

pub fn default_checker_eps_abs(
    _input: Input,
    expected: Option<Input>,
    actual: Input,
) -> Result<(), String> {
    default_checker_eps(expected, actual, |expected, actual| {
        (expected - actual).abs() < EPS.with(|eps| eps.get())
    })
}
