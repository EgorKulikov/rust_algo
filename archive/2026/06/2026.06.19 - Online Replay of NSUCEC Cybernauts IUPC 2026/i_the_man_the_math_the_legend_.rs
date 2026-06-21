use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::rational::Rational;
use algo_lib::string::str::StrReader;
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let d = input.read_long();
    let l = input.read_long();
    let r = input.read_long();

    if b == 0 {
        if c == 0 {
            out.print_line("None");
            return;
        }
        let val = Rational::new(a, c) - Rational::new_int(d);
        match val.cmp(&Rational::zero()) {
            Ordering::Less => {
                out.print_line("Jewel");
            }
            Ordering::Equal => {
                out.print_line("None");
            }
            Ordering::Greater => {
                out.print_line("Rasel");
            }
        }
        return;
    }
    let inf = -Rational::new(c, b);
    let mut rasel = Rational::zero();
    let mut jewel = Rational::zero();
    let calc = |x: Rational<i64>| {
        Rational::new_int(a) / (x * Rational::new_int(b) + Rational::new_int(c))
            - Rational::new_int(d)
    };
    let mut solve = |from: Rational<i64>, to: Rational<i64>| {
        if l == r {
            loop {}
        }
        if calc(from) == Rational::zero() || d == 0 {
            let val = calc((from + to) / Rational::new_int(2));
            assert_ne!(val, Rational::zero());
            if val > Rational::zero() {
                rasel += (to - from).abs();
            } else {
                jewel += (to - from).abs();
            }
            return;
        }
        let target = Rational::new(a - d * c, d * b);
        if from < target && target < to || from > target && target > to {
            let val = calc(from);
            assert_ne!(val, Rational::zero());
            if val > Rational::zero() {
                rasel += (target - from).abs();
                jewel += (to - target).abs();
            } else {
                jewel += (target - from).abs();
                rasel += (to - target).abs();
            }
        } else {
            let val = calc((from + to) / Rational::new_int(2));
            assert_ne!(val, Rational::zero());
            if val > Rational::zero() {
                rasel += (to - from).abs();
            } else {
                jewel += (to - from).abs();
            }
        }
    };
    if l == r {
        if inf == Rational::new_int(l) {
            out.print_line("None");
        } else {
            match calc(Rational::new_int(l)).cmp(&Rational::zero()) {
                Ordering::Less => out.print_line("Jewel"),
                Ordering::Equal => out.print_line("None"),
                Ordering::Greater => out.print_line("Rasel"),
            }
        }
        return;
    }
    if a == 0 {
        match d.cmp(&0) {
            Ordering::Greater => out.print_line("Jewel"),
            Ordering::Equal => out.print_line("None"),
            Ordering::Less => out.print_line("Rasel"),
        }
        return;
    }
    if Rational::new_int(l) == inf {
        solve(Rational::new_int(r), Rational::new_int(l));
    } else if Rational::new_int(r) == inf {
        solve(Rational::new_int(l), Rational::new_int(r));
    } else if Rational::new_int(l) < inf && Rational::new_int(r) > inf {
        solve(Rational::new_int(l), inf);
        solve(Rational::new_int(r), inf);
    } else {
        solve(Rational::new_int(l), Rational::new_int(r));
    }

    match rasel.cmp(&jewel) {
        Ordering::Less => out.print_line("Jewel"),
        Ordering::Equal => out.print_line("None"),
        Ordering::Greater => out.print_line("Rasel"),
    }
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
