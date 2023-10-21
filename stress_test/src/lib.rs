use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::{err, Output};
use algo_lib::misc::random::random;
use algo_lib::numbers::gauss::invert;
use algo_lib::numbers::mod_int::ModInt7;
use std::io::{stdout, Write};

fn generate_test(out: &mut Output) {
    let n = 500;
    type Mod = ModInt7;
    let a = Arr2d::generate(n, n, |_, _| Mod::new(random().next(1000) as i32));
    let mut b = invert(&a).unwrap();
    let rand = random();
    let m = rand.next(13) as usize;
    for _ in 0..m {
        let i = rand.next(n as u64) as usize;
        let j = rand.next(n as u64) as usize;
        let v = Mod::new(rand.next(1000) as i32);
        b[(i, j)] = v;
    }
    err().print_line(("Changed", m));
    out.print_line(n);
    out.print_line(a);
    out.print_line(b);
}

fn stupid(_input: &mut Input, _out: &mut Output) {}

pub fn stress_test(
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
