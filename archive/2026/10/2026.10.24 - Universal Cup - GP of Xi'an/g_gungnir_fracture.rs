use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let a = input.read_long_vec(1 << n);
    let mut rec = RecursiveFunction2::new(|rec, l: usize, r: usize| -> (i64, i64, i64) {
        if l + 1 == r {
            (a[l], a[l], 0)
        } else {
            let mid = (l + r) / 2;
            let (mut lf0, mut lf1, mut left_partial) = rec.call(l, mid);
            let (mut rf0, mut rf1, mut right_partial) = rec.call(mid, r);
            let mut partial = 0;
            if lf1 < rf0 {
                let delta = rf0 - lf1;
                left_partial = (left_partial - delta).max(0);
                partial += delta;
                rf0 = lf1;
                rf1 = lf1;
            }
            if rf1 < lf0 {
                let delta = lf0 - rf1;
                right_partial = (right_partial - delta).max(0);
                partial += delta;
                lf0 = rf1;
                lf1 = rf1;
            }
            lf1.minim(rf1);
            rf1.minim(lf1);
            if lf0 < rf0 {
                let delta = left_partial.min(rf0 - lf0);
                partial += delta;
                left_partial -= delta;
                rf0 -= delta;
            }
            if rf0 < lf0 {
                let delta = right_partial.min(lf0 - rf0);
                partial += delta;
                right_partial -= delta;
                lf0 -= delta;
            }
            lf0.maxim(rf0);
            rf0.minim(lf0);
            let remove_full = lf0.min(left_partial).min(right_partial);
            lf0 -= remove_full;
            (lf0, lf1, left_partial + right_partial + partial)
        }
    });
    let (ans_full_min, _, ans_partial) = rec.call(0, 1 << n);
    out.print_line(ans_full_min + ans_partial);
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
