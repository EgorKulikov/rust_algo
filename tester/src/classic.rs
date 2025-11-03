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
            let checker_result = if let Some(expected) = expected {
                (checker)(
                    Input::slice(input),
                    Some(Input::slice(expected)),
                    Input::slice(output.as_slice()),
                )
            } else {
                (checker)(Input::slice(input), None, Input::slice(output.as_slice()))
            };
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
                        second_duration: None,
                        input_exhausted: is_exhausted,
                    },
                    output,
                )
            } else {
                (
                    Outcome::OK {
                        duration,
                        second_duration: None,
                        input_exhausted: is_exhausted,
                    },
                    output,
                )
            }
        }
        Err(err) => (process_error(err), Vec::new()),
    };
    if test_set.print_details() || !matches!(outcome, Outcome::OK { .. }) {
        print_output(tester.trim(&output), true, 0);
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
    pub static EPS: Cell<f64> = Cell::new(1e-9);
}

pub fn default_checker_eps_rel(
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
                let expected_token = String::from_utf8(expected_token.unwrap()).unwrap();
                let actual_token =
                    String::from_utf8(actual_token.as_ref().unwrap().clone()).unwrap();
                let mut failed = true;
                if let Ok(expected_token) = expected_token.parse::<f64>() {
                    if let Ok(actual_token) = actual_token.parse::<f64>() {
                        let by = expected_token.abs().max(1.);
                        // 1.73
                        if (expected_token - actual_token).abs() < EPS.with(|eps| eps.get()) * by {
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
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}

pub fn default_checker_eps_abs(
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
                let expected_token = String::from_utf8(expected_token.unwrap()).unwrap();
                let actual_token =
                    String::from_utf8(actual_token.as_ref().unwrap().clone()).unwrap();
                let mut failed = true;
                if let Ok(expected_token) = expected_token.parse::<f64>() {
                    if let Ok(actual_token) = actual_token.parse::<f64>() {
                        // 1.73
                        if (expected_token - actual_token).abs() < EPS.with(|eps| eps.get()) {
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
        token_num += 1;
        if actual_token.is_none() {
            break;
        }
    }
    Ok(())
}
