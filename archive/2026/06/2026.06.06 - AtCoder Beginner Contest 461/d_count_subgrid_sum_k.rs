use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let kk = input.read_int();
    let s = input.read_char_table(h, w);

    let mut d = Arr2d::new(h + 1, w + 1, 0);
    for i in 0..h {
        for j in 0..w {
            d[(i + 1, j + 1)] =
                d[(i + 1, j)] + d[(i, j + 1)] - d[(i, j)] + (s[(i, j)] - b'0') as i32;
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in i + 1..=h {
            for k in 0..w {
                let mut left = k + 1;
                let mut right = w + 1;
                while left < right {
                    let mid = (left + right) / 2;
                    if d[(j, mid)] - d[(i, mid)] + d[(i, k)] - d[(j, k)] < kk {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                let from = left;
                right = w + 1;
                while left < right {
                    let mid = (left + right) / 2;
                    if d[(j, mid)] - d[(i, mid)] + d[(i, k)] - d[(j, k)] <= kk {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }
                ans += left - from;
            }
        }
    }
    out.print_line(ans);
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
