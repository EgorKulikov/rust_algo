//{"name":"A - Loading Cargo","group":"AtCoder - AtCoder Weekday Contest 0014 Beta","url":"https://atcoder.jp/contests/awc0014/tasks/awc0014_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5 20\n3 5 7\n","output":"5 4 2\n"},{"input":"4 10 15\n1 4 8 16\n","output":"10 3 1 0\n"},{"input":"10 100 1000\n1 2 5 10 50 100 200 333 500 1001\n","output":"100 100 100 100 20 10 5 3 2 0\n"},{"input":"20 1000000000 1000000000000000000\n1 2 3 10 100 1000 10000 100000 1000000 10000000 100000000 1000000000 999999999 500000000 250000000 123456789 777777777 42 314159265 271828182\n","output":"1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000\n"},{"input":"1 1000000000 1\n1000000000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_long();
    let t = input.read_long();
    let p = input.read_long_vec(n);

    out.print_line_iter(p.iter_map(|x| (t / x).min(r)));
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
