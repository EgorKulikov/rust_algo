#![allow(unused_variables)]
#![allow(dead_code)]

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::io::{stdout, Write};

fn generate_test(out: &mut Output) {}

fn stupid(input: &mut Input, out: &mut Output) {}

pub fn check(input: &mut &[u8], expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
    let mut _input = Input::new(input);
    let mut expected = Input::new(expected);
    let mut actual = Input::new(actual);
    let mut token_num = 0usize;
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

pub(crate) fn run_tests() -> bool {
    let blue = "\x1B[34m";
    let red = "\x1B[31m";
    let green = "\x1B[32m";
    let yellow = "\x1B[33m";
    let def = "\x1B[0m";
    let time_limit = std::time::Duration::from_millis($TIME_LIMIT);
    let mut paths = std::fs::read_dir("./$TASK/tests/")
        .unwrap()
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    paths.sort_by(|a, b| a.path().cmp(&b.path()));
    let mut test_failed = 0usize;
    let mut test_total = 0usize;
    for path in paths {
        let sub_path = path;
        if sub_path.file_type().unwrap().is_file() {
            let path = sub_path.path();
            match path.extension() {
                None => {}
                Some(extension) => {
                    if extension.to_str() == Some("in") {
                        println!("=====================================================");
                        test_total += 1;
                        let name = path.file_name().unwrap().to_str().unwrap();
                        let name = &name[..name.len() - 3];
                        println!("{}Test {}{}", blue, name, def);
                        println!("{}Input:{}", blue, def);
                        let input = std::fs::read_to_string(&path).unwrap();
                        println!("{}", input);
                        let expected = match std::fs::read_to_string(
                            path.parent().unwrap().join(format!("{}.out", name)),
                        ) {
                            Ok(res) => Some(res),
                            Err(_) => None,
                        };
                        println!("{}Expected:{}", blue, def);
                        match &expected {
                            None => {
                                println!("{}Not provided{}", yellow, def);
                            }
                            Some(expected) => {
                                println!("{}", expected);
                            }
                        }
                        println!("{}Output:{}", blue, def);
                        match std::panic::catch_unwind(|| {
                            let mut file = std::fs::File::open(&path).unwrap();
                            let started = std::time::Instant::now();
                            let mut output = Vec::new();
                            let is_exhausted = crate::run(Input::new(&mut file), Output::new(&mut output));
                            let res = started.elapsed();
                            println!("{}", String::from_utf8_lossy(&output));
                            (output, res, is_exhausted)
                        }) {
                            Ok((output, duration, is_exhausted)) => {
                                println!(
                                    "{}Time elapsed: {:.3}s{}",
                                    blue,
                                    (duration.as_millis() as f64) / 1000.,
                                    def,
                                );
                                if !is_exhausted {
                                    println!("{}Input not exhausted{}", red, def);
                                }
                                if let Some(expected) = expected {
                                    let mut expected_bytes = expected.as_bytes().clone();
                                    match check(
                                        &mut input.as_bytes(),
                                        &mut expected_bytes,
                                        &mut &output[..],
                                    ) {
                                        Ok(_) => {}
                                        Err(err) => {
                                            println!(
                                                "{}Verdict: {}Wrong Answer ({}){}",
                                                blue, red, err, def
                                            );
                                            test_failed += 1;
                                            continue;
                                        }
                                    }
                                }
                                if duration > time_limit {
                                    test_failed += 1;
                                    println!("{}Verdict: {}Time Limit{}", blue, red, def);
                                } else {
                                    println!("{}Verdict: {}OK{}", blue, green, def)
                                }
                            }
                            Err(err) => {
                                test_failed += 1;
                                match err.downcast::<&str>() {
                                    Ok(as_string) => println!(
                                        "{}Verdict: {}RuntimeError ({:?}){}",
                                        blue, red, as_string, def
                                    ),
                                    Err(err) => println!(
                                        "{}Verdict: {}RuntimeError ({:?}){}",
                                        blue, red, err, def
                                    ),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if test_failed == 0 {
        println!(
            "{}All {}{}{} tests passed{}",
            blue, green, test_total, blue, def
        );
    } else {
        println!(
            "{}{}/{}{} tests failed{}",
            red, test_failed, test_total, blue, def
        );
    }
    test_failed == 0
}

pub(crate) fn stress_test(
    solution: impl Fn(Input, Output) -> bool,
    checker: impl Fn(&mut &[u8], &mut &[u8], &mut &[u8]) -> Result<(), String>,
) {
    fn generate_test_int() -> Vec<u8> {
        let mut res = Vec::new();
        let mut output = Output::new(&mut res);
        generate_test(&mut output);
        output.flush();
        res
    }

    fn stupid_int(mut input: &[u8]) -> Vec<u8> {
        let mut input = Input::new(&mut input);
        let mut res = Vec::new();
        let mut output = Output::new(&mut res);
        stupid(&mut input, &mut output);
        output.flush();
        res
    }

    fn actual(mut input: &[u8], solution: &impl Fn(Input, Output) -> bool) -> Vec<u8> {
        let input = Input::new(&mut input);
        let mut res = Vec::new();
        let output = Output::new(&mut res);
        solution(input, output);
        res
    }

    println!("");
    println!("\x1B[34mStress testing\x1B[0m");
    loop {
        let input = generate_test_int();
        let expected = stupid_int(&input);
        let actual = actual(&input, &solution);
        print!("#");
        stdout().flush().unwrap();
        let res = checker(
            &mut input.as_slice(),
            &mut expected.as_slice(),
            &mut actual.as_slice(),
        );
        match res {
            Ok(_) => {
                print!(".");
                stdout().flush().unwrap();
            }
            Err(err) => {
                println!();
                println!("Failed on test:\n{}", String::from_utf8(input).unwrap());
                println!("Expected:\n{}", String::from_utf8(expected).unwrap());
                println!("Actual:\n{}", String::from_utf8(actual).unwrap());
                println!("Error: {}", err);
                panic!();
            }
        }
    }
}
