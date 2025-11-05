use crate::print::{print_diff, print_output};
use crate::test_set::TestSet;
use crate::{process_error, Outcome, Tester};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

pub(crate) fn run_single_test_run_twice<T: TestSet>(
    tester: &Tester,
    mixer: fn(Input, Input, Option<Input>, Output, Output) -> Result<bool, String>,
    checker: fn(Input, Option<Input>, Input) -> Result<(), String>,
    input: &[u8],
    expected: Option<&[u8]>,
    test_set: &T,
    test_id: &T::TestId,
) -> Outcome {
    let input = &input;
    let (outcome, output, new_input, new_expected) = match std::panic::catch_unwind(|| {
        let input = *input;
        let started = std::time::Instant::now();
        let mut output = Vec::new();
        let is_exhausted = (tester.solution)(Input::slice(input), Output::buf(&mut output));
        let res = started.elapsed();
        (output, res, is_exhausted)
    }) {
        Ok((output, duration, is_exhausted)) => {
            test_set.save_first_output(test_id, &output);
            let input = *input;
            let mut new_input = Vec::new();
            let mut new_expected = Vec::new();
            let mixer_result = mixer(
                Input::slice(input),
                Input::slice(output.as_slice()),
                expected.map(|v| Input::slice(v)),
                Output::buf(&mut new_input),
                Output::buf(&mut new_expected),
            );
            if let Err(mixer_output) = mixer_result {
                (
                    Outcome::WrongAnswer {
                        input_exhausted: is_exhausted,
                        checker_output: mixer_output,
                    },
                    output,
                    Vec::new(),
                    None,
                )
            } else if duration.as_millis() as u64 > tester.time_limit {
                (
                    Outcome::TimeLimit {
                        duration,
                        second_duration: None,
                        input_exhausted: is_exhausted,
                    },
                    output,
                    new_input,
                    if mixer_result.unwrap() {
                        Some(new_expected)
                    } else {
                        None
                    },
                )
            } else {
                (
                    Outcome::OK {
                        duration,
                        second_duration: None,
                        input_exhausted: is_exhausted,
                    },
                    output,
                    new_input,
                    if mixer_result.unwrap() {
                        Some(new_expected)
                    } else {
                        None
                    },
                )
            }
        }
        Err(err) => (process_error(err), Vec::new(), Vec::new(), None),
    };
    if test_set.print_details() || !matches!(outcome, Outcome::OK { .. }) {
        print_output(tester.trim(&output), true, 1);
    }
    if test_set.print_details() {
        println!("{}Input #2:{}", crate::print::BLUE, crate::print::DEF);
        println!("{}", String::from_utf8_lossy(new_input.as_slice()));
    }
    if !matches!(outcome, Outcome::OK { .. }) && !matches!(outcome, Outcome::TimeLimit { .. }) {
        return outcome;
    }
    test_set.save_second_input(test_id, &new_input);
    let (mut second_outcome, output) = match std::panic::catch_unwind(|| {
        let input = new_input.as_slice();
        let started = std::time::Instant::now();
        let mut output = Vec::new();
        let is_exhausted = (tester.solution)(Input::slice(input), Output::buf(&mut output));
        let res = started.elapsed();
        (output, res, is_exhausted)
    }) {
        Ok((output, duration, is_exhausted)) => {
            test_set.save_output(test_id, &output);
            let input = *input;
            let checker_result = if let Some(expected) = new_expected {
                (checker)(
                    Input::slice(input),
                    Some(Input::slice(expected.as_slice())),
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
    if test_set.print_details() || !matches!(second_outcome, Outcome::OK { .. }) {
        print_output(tester.trim(&output), true, 2);
    }
    if test_set.print_details() && matches!(second_outcome, Outcome::WrongAnswer { .. }) {
        print_diff(test_set, test_id);
    }
    match outcome {
        Outcome::OK { duration, .. } => match &mut second_outcome {
            Outcome::OK {
                duration: sd,
                second_duration,
                ..
            } => {
                *second_duration = Some(*sd);
                *sd = duration;
            }
            Outcome::TimeLimit {
                duration: sd,
                second_duration,
                ..
            } => {
                *second_duration = Some(*sd);
                *sd = duration;
            }
            _ => {}
        },
        Outcome::TimeLimit { duration, .. } => match &mut second_outcome {
            Outcome::OK {
                duration: sd,
                input_exhausted,
                ..
            } => {
                let sd = *sd;
                let ie = *input_exhausted;
                second_outcome = Outcome::TimeLimit {
                    duration,
                    second_duration: Some(sd),
                    input_exhausted: ie,
                }
            }
            Outcome::TimeLimit {
                duration: sd,
                second_duration,
                ..
            } => {
                *second_duration = Some(*sd);
                *sd = duration;
            }
            _ => {}
        },
        _ => unreachable!(),
    }
    second_outcome
}
