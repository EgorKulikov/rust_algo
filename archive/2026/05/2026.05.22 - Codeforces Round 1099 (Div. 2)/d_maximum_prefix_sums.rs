use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let mut a = input.read_long_vec(n);
    let c = input.read_long_vec(n);

    if s[0] == b'0' {
        a[0] = c[0];
    }
    if a[0] != c[0] {
        out.print_line(false);
        return;
    }
    let mut flexible = None;
    let mut sum = c[0];
    for i in 1..n {
        let prev = c[i - 1];
        let delta = c[i] - prev;
        if delta < 0 {
            out.print_line(false);
            return;
        }
        if delta == 0 {
            if s[i] == b'1' {
                let excess = -c[i] + sum + a[i];
                if excess > 0 {
                    if let Some(flexible) = flexible {
                        a[flexible] -= excess;
                    } else {
                        out.print_line(false);
                        return;
                    }
                    sum = c[i];
                } else {
                    sum += a[i];
                }
            } else {
                flexible = Some(i);
            }
        } else {
            let diff = c[i] - sum;
            if s[i] == b'1' {
                if delta > a[i] {
                    out.print_line(false);
                    return;
                } else if diff != a[i] {
                    if let Some(flexible) = flexible {
                        a[flexible] -= a[i] - diff;
                    }
                }
            } else {
                a[i] = diff;
            }
            flexible = None;
            sum = c[i];
        }
    }
    let mut max = None;
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        max.maxim(sum);
        if max != Some(c[i]) {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
    out.print_line(a);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
