//{"name":"Gift","group":"Eolymp - Basecamp - Weekend Practice #17","url":"https://eolymp.com/en/compete/68ise1j0d17fp04bqe407psjqg/problem/3","interactive":false,"timeLimit":15000,"tests":[{"input":"5 5\n0 0 0 0 0\n0 0 0 0 1\n0 0 0 0 1\n0 0 0 0 1\n0 1 1 1 1\n","output":"-1\n"},{"input":"5 5\n1 0 1 0 0\n0 1 1 0 1\n1 1 1 0 1\n0 0 0 1 1\n0 1 1 1 1\n","output":"2\n"},{"input":"4 5\n1 0 1 0 1\n1 1 1 0 0\n0 1 1 1 1\n0 0 1 1 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_table(n, m);

    let mut ans = None;
    for x in 0..2 {
        let mut row = vec![0; n];
        let mut col = vec![0; m];
        row[0] = x;
        for i in 0..m.upper_div(2) {
            let v0 = a[(0, i)] ^ row[0];
            let v1 = a[(n - 1, m - 1 - i)] ^ row[n - 1];
            col[i] = v0 ^ v1;
        }
        for i in 1..n.upper_div(2) {
            let v0 = a[(i, 0)] ^ col[0];
            let v1 = a[(n - 1 - i, m - 1)] ^ col[m - 1];
            row[i] = v0 ^ v1;
        }
        let mut good = true;
        for (i, j) in a.indices() {
            let v0 = a[(i, j)] ^ row[i] ^ col[j];
            let v1 = a[(n - 1 - i, m - 1 - j)] ^ row[n - 1 - i] ^ col[m - 1 - j];
            if v0 != v1 {
                good = false;
                break;
            }
        }
        if good {
            ans.minim(row.copy_sum() + col.copy_sum());
        }
    }
    out.print_line(ans);
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
