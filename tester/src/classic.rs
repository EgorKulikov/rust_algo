use crate::print::{print_diff, print_output};
use crate::test_set::TestSet;
use crate::{process_error, Outcome, Tester};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

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
        let mut input = *input;
        let started = std::time::Instant::now();
        let mut output = Vec::new();
        let is_exhausted = (tester.solution)(Input::new(&mut input), Output::new(&mut output));
        let res = started.elapsed();
        (output, res, is_exhausted)
    }) {
        Ok((output, duration, is_exhausted)) => {
            test_set.save_output(test_id, &output);
            let mut input = *input;
            let checker_result = if let Some(mut expected) = expected {
                (checker)(
                    Input::new(&mut input),
                    Some(Input::new(&mut expected)),
                    Input::new(&mut output.as_slice()),
                )
            } else {
                (checker)(
                    Input::new(&mut input),
                    None,
                    Input::new(&mut output.as_slice()),
                )
            };
            if let Err(checker_output) = checker_result {
                test_set.output_diff(test_id);
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
