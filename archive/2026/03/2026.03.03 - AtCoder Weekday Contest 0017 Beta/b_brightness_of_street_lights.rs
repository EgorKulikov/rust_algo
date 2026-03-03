//{"name":"B - Brightness of Street Lights","group":"AtCoder - AtCoder Weekday Contest 0017 Beta","url":"https://atcoder.jp/contests/awc0017/tasks/awc0017_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 2 3 4 5\n2 4 4\n","output":"2 3 6 6 7\n"},{"input":"7 5\n0 0 0 0 0 0 0\n1 3 5 7 4\n","output":"1 2 2 3 2 2 1\n"},{"input":"10 8\n100 200 300 400 500 600 700 800 900 1000\n1 1 5 5 5 10 3 7\n","output":"102 203 301 404 503 604 701 801 901 1001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_int_vec(n);
    let b = input.read_size_vec(m).dec();

    for i in b {
        for j in i.saturating_sub(1)..=(i + 1).min(n - 1) {
            a[j] += 1;
        }
    }
    out.print_line(a);
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
