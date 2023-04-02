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

pub fn stress_test(
    solution: impl Fn(Input) -> bool,
    checker: impl Fn(&mut &[u8], &mut &[u8]) -> Result<(), String>,
) {
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

    fn actual(mut input: &[u8], solution: &impl Fn(Input) -> bool) -> Vec<u8> {
        let input = Input::new(&mut input);
        unsafe {
            OUTPUT = Some(Output::new(Box::new(WriteDelegate {})));
        }
        solution(input);
        output().flush();
        take_output()
    }

    println!("");
    println!("\x1B[34mStress testing\x1B[0m");
    loop {
        let input = generate_test_int();
        let expected = stupid_int(&input);
        let actual = actual(&input, &solution);
        let res = checker(&mut expected.as_slice(), &mut actual.as_slice());
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
