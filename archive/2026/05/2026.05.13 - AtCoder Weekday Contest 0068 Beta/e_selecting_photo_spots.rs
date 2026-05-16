//{"name":"E - Selecting Photo Spots","group":"AtCoder - AtCoder Weekday Contest 0068 Beta","url":"https://atcoder.jp/contests/awc0068/tasks/awc0068_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n1 2 3\n4 5 6\n7 8 9\n","output":"19\n"},{"input":"4 4\n0 2 5 1\n4 0 3 6\n7 8 0 2\n1 9 4 0\n","output":"43\n"},{"input":"5 3\n8 1 6 2 9\n3 5 7 4 1\n9 2 4 8 3\n6 0 5 7 2\n1 8 3 9 4\n","output":"37\n"},{"input":"8 4\n12 7 3 15 8 6 14 2\n9 11 5 4 13 10 1 16\n6 14 2 12 7 15 3 8\n10 1 16 9 5 11 4 13\n8 13 7 3 14 2 12 6\n4 15 10 1 16 9 5 11\n14 2 12 6 8 13 7 3\n5 11 4 16 10 1 15 9\n","output":"132\n"},{"input":"1 1\n1000000000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::sliding_window::SlidingWindow;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_table(n, n);

    let mut b_sum = Arr2d::new(n, n - k + 1, 0);
    let mut b_max = Arr2d::new(n, n - k + 1, 0);
    for i in 0..n {
        let mut sw = SlidingWindow::new(k, i64::max);
        let mut sum = 0;
        for j in 0..k - 1 {
            sum += a[(i, j)];
            sw.push(a[(i, j)]);
        }
        for j in k - 1..n {
            sum += a[(i, j)];
            sw.push(a[(i, j)]);
            b_sum[(i, j + 1 - k)] = sum;
            b_max[(i, j + 1 - k)] = sw.get();
            sum -= a[(i, j + 1 - k)];
        }
    }
    let mut ans = None;
    for i in 0..=n - k {
        let mut sw = SlidingWindow::new(k, i64::max);
        let mut sum = 0;
        for j in 0..k - 1 {
            sum += b_sum[(j, i)];
            sw.push(b_max[(j, i)]);
        }
        for j in k - 1..n {
            sum += b_sum[(j, i)];
            sw.push(b_max[(j, i)]);
            ans.maxim(sum - sw.get());
            sum -= b_sum[(j + 1 - k, i)];
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
