use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;
use std::io::{stdout, Write};

fn generate_test(out: &mut Output) {
    let a = random().next(100) + 1;
    let b = random().next(100) + 1;
    out.print_line((a, b));
}

fn stupid(input: &mut Input, out: &mut Output) {
    let a = input.read_size();
    let b = input.read_size();
    out.print_line(a + b);
}

pub fn stress_test(
    solution: impl Fn(Input, Output) -> bool,
    checker: impl Fn(&mut &[u8], &mut &[u8]) -> Result<(), String>,
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
