//{"name":"A - Over Budget","group":"AtCoder - AtCoder Weekday Contest 0016 Beta","url":"https://atcoder.jp/contests/awc0016/tasks/awc0016_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n100 80\n50 50\n30 60\n","output":"1 20\n"},{"input":"5\n500 300\n200 400\n150 100\n600 600\n800 350\n","output":"3 700\n"},{"input":"10\n1000000000 999999999\n500000000 500000000\n300000000 100000000\n750000000 800000000\n999999999 1\n100 200\n450000000 300000000\n600000000 600000001\n1 1000000000\n888888888 777777777\n","output":"5 1461111110\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let ab = input.read_long_pair_vec(n);

    let mut qty = 0;
    let mut sum = 0;
    for (a, b) in ab {
        if a > b {
            qty += 1;
            sum += a - b;
        }
    }
    out.print_line((qty, sum));
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
