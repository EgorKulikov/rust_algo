//{"name":"B - Marathon","group":"AtCoder - AtCoder Weekday Contest 0014 Beta","url":"https://atcoder.jp/contests/awc0014/tasks/awc0014_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 10\n30 50 40 60\n4 10 15 20\n","output":"2 3 4 5\n"},{"input":"4 5\n100 100 100\n10 20 30\n","output":"-1\n"},{"input":"8 100\n500 300 700 200 400 600 800\n4 8 20 25 35 50 80\n","output":"4 5 6 7 8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let v = input.read_long();
    let d = input.read_long_vec(n - 1);
    let t = input.read_long_vec(n - 1);

    let mut ans = Vec::new();
    let mut sum = 0;
    for i in 0..n - 1 {
        sum += d[i];
        if sum < t[i].saturating_mul(v) {
            ans.push(i + 2);
        }
    }
    if ans.is_empty() {
        out.print_line(-1);
    } else {
        out.print_line(ans);
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
