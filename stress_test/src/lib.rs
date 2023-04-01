use algo_lib::io::input::Input;
use algo_lib::io::output::{output, Output, OUTPUT};
use algo_lib::misc::random::random;
use algo_lib::out_line;
use std::io::{stdout, Write};
use std::mem::swap;

fn generate_test() {
    let n = random().next(100) + 1;
    out_line!(n);
}

fn stupid(input: &mut Input) {
    let n = input.read_size();
    out_line!(n * n);
}

fn check(mut expected: Input, mut actual: Input) -> Result<(), String> {
    check_int(&mut expected, &mut actual)
}

pub fn stress_test(f: impl Fn(Input) -> bool) {
    fn take_output() -> Vec<u8> {
        let mut res = Vec::new();
        unsafe {
            swap(&mut res, &mut OUT);
        }
        res
    }

    fn generate_test_int() -> Vec<u8> {
        unsafe {
            OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
        }
        generate_test();
        output().flush();
        take_output()
    }

    fn stupid_int(mut input: &[u8]) -> Vec<u8> {
        let mut input = Input::new(&mut input);
        unsafe {
            OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
        }
        stupid(&mut input);
        output().flush();
        take_output()
    }

    fn check_int(expected: &mut &[u8], actual: &mut &[u8]) -> Result<(), String> {
        let expected = Input::new(expected);
        let actual = Input::new(actual);
        check(expected, actual)
    }

    fn actual(mut input: &[u8], f: &impl Fn(Input) -> bool) -> Vec<u8> {
        let input = Input::new(&mut input);
        unsafe {
            OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
        }
        f(input);
        output().flush();
        take_output()
    }

    loop {
        let input = generate_test_int();
        let expected = stupid_int(&input);
        let actual = actual(&input, &f);
        let res = check_int(&mut expected.as_slice(), &mut actual.as_slice());
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

static mut OUT: Vec<u8> = Vec::new();

struct WriteDelegate {}

impl Write for WriteDelegate {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unsafe {
            OUT.append(&mut Vec::from(buf));
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn check_int(expected: &mut Input, actual: &mut Input) -> Result<(), String> {
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
