use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::real::{Real, RealReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let w = input.read_real();
    let x1 = input.read_real();
    let x2 = input.read_real();
    let y = input.read_real();
    let u = input.read_real();
    let v = input.read_real();

    let base = y / v;
    let xx1 = x1 + u * base;
    let xx2 = x2 + u * base;
    if xx1 >= Real(0.) || xx2 <= Real(0.) {
        out.print_line(w / v);
        return;
    }
    let mut ans = Real(f64::INFINITY);
    for x in [x1, x2] {
        let a = u * u - v * v;
        let b = x * u * 2;
        let c = x * x + y * y;
        if a == Real(0.) {
            let t = -c / b;
            if t >= Real(0.) {
                ans.minim(t);
            }
        } else {
            let d = b * b - a * c * 4;
            if d < Real(0.) {
                continue;
            }
            let t1 = (-b + d.sqrt()) / (a * 2);
            let t2 = (-b - d.sqrt()) / (a * 2);
            if t1 >= Real(0.) {
                ans.minim(t1);
            }
            if t2 >= Real(0.) {
                ans.minim(t2);
            }
        }
    }
    ans += (w - y) / v;
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
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
