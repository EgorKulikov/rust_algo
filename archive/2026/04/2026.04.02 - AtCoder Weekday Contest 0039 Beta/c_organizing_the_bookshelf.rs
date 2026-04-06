//{"name":"C - Organizing the Bookshelf","group":"AtCoder - AtCoder Weekday Contest 0039 Beta","url":"https://atcoder.jp/contests/awc0039/tasks/awc0039_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 10\n3 5 2 8 1\n4 3 2 5 6\n","output":"15\n"},{"input":"8 20\n1 4 6 2 10 3 7 5\n5 3 4 8 6 2 3 9\n","output":"25\n"},{"input":"10 1000000000000000\n100 200 300 400 500 600 700 800 900 1000\n1 1 1 1 1 1 1 1 1 1\n","output":"5500\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);

    let mut tw = 0;
    let mut ts = 0;
    let mut at = 0;
    let mut ans = 0;
    for i in 0..n {
        while at < n && tw <= k {
            ans.maxim(ts);
            ts += a[at];
            tw += b[at];
            at += 1;
        }
        if tw <= k {
            ans.maxim(ts);
        }
        ts -= a[i];
        tw -= b[i];
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
