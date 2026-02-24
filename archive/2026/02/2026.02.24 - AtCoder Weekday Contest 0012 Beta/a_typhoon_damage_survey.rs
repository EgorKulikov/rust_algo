//{"name":"A - Typhoon Damage Survey","group":"AtCoder - AtCoder Weekday Contest 0012 Beta","url":"https://atcoder.jp/contests/awc0012/tasks/awc0012_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3\n1 5 3 4 2\n10 20 30 40 50\n","output":"90\n"},{"input":"4 2\n5 10 8 3\n100 200 300 400\n","output":"0\n"},{"input":"10 50\n10 20 30 40 50 60 70 80 90 100\n5 15 25 35 45 55 65 75 85 95\n","output":"125\n"},{"input":"20 500\n100 200 300 400 500 600 700 800 900 1000 150 250 350 450 550 650 750 850 950 50\n10 20 30 40 50 60 70 80 90 100 15 25 35 45 55 65 75 85 95 5\n","output":"275\n"},{"input":"1 1000000000\n1000000000\n999999999\n","output":"999999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_int();
    let h = input.read_int_vec(n);
    let c = input.read_long_vec(n);

    let mut ans = 0;
    for i in 0..n {
        if h[i] <= t {
            ans += c[i];
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
