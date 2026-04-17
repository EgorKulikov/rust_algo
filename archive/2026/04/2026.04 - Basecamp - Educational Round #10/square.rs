//{"name":"Square","group":"Eolymp - Basecamp - Educational Round #10","url":"https://eolymp.com/en/compete/lce8ibks056gvennio8eubldr0/problem/7","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 0 1 0 1 0\n0 1 0 1 0 1\n1 0 1 0 1 0\n0 1 0 1 0 1\n1 0 1 0 1 0\n0 1 0 1 0 1\n","output":"18\n"},{"input":"6\n0 1 1 1 1 0\n1 0 1 1 1 1\n0 1 1 1 1 1\n1 1 0 1 1 1\n1 1 1 1 0 1\n1 1 0 1 1 1\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_int_table(n, n);

    let mut a = Arr2d::new(n + 1, n + 1, 0);
    for (i, j) in t.indices() {
        a[(i + 1, j + 1)] = 1 - t[(i, j)] + a[(i, j + 1)] + a[(i + 1, j)] - a[(i, j)];
    }
    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut found = false;
        'outer: for i in 0..=n - mid {
            for j in 0..=n - mid {
                if a[(i + mid, j + mid)] - a[(i, j + mid)] - a[(i + mid, j)] + a[(i, j)] == 0 {
                    found = true;
                    break 'outer;
                }
            }
        }
        if found {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    let mut qty = 0;
    for i in 0..=n - left {
        for j in 0..=n - left {
            if a[(i + left, j + left)] - a[(i, j + left)] - a[(i + left, j)] + a[(i, j)] == 0 {
                qty += 1;
            }
        }
    }
    out.print_line(left * qty);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
