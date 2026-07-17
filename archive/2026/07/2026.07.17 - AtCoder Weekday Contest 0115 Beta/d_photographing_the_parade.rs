use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_long();
    let q = input.read_long();
    let mut xvl = input.read_vec::<(i64, i64, i64)>(n);

    fn go(xvl: &[(i64, i64, i64)], d: i64) -> i64 {
        let single = |d: i64| -> i64 {
            let mut min = i64::MAX;
            let mut max = i64::MIN;
            for &(x, v, l) in xvl {
                min.minim(x + v * d - l);
                max.maxim(x + v * d + l);
            }
            max - min
        };
        let mut left = 0;
        let mut right = d;
        while right - left > 3 {
            let mid_left = (2 * left + right) / 3;
            let mid_right = (2 * right + left) / 3;
            if single(mid_left) < single(mid_right) {
                right = mid_right;
            } else {
                left = mid_left;
            }
        }
        let mut result = i64::MAX;
        for i in left..=right {
            result = result.min(single(i));
        }
        result
    }
    out.print_line(go(&xvl, d));
    for _ in 0..q {
        let p = input.read_size() - 1;
        xvl[p] = input.read();
        out.print_line(go(&xvl, d));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
