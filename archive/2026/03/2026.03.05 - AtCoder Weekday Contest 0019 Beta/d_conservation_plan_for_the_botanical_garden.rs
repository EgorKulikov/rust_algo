//{"name":"D - Conservation Plan for the Botanical Garden","group":"AtCoder - AtCoder Weekday Contest 0019 Beta","url":"https://atcoder.jp/contests/awc0019/tasks/awc0019_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 100 50\n80 30 60\n50 60 40\n30 40 50\n","output":"130\n"},{"input":"5 200 100\n100 50 80\n200 150 120\n150 80 90\n50 200 30\n80 90 70\n","output":"500\n"},{"input":"10 500 1000\n500 800 100\n300 1200 80\n450 500 150\n200 2000 50\n150 900 120\n600 600 200\n100 1500 40\n350 400 180\n250 1100 90\n400 700 160\n","output":"2400\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let t = input.read_int();
    let abc = input.read_vec::<(i32, i32, usize)>(n);

    let mut ans = vec![0; m + 1];
    let mut add = 0;
    for (a, b, c) in abc {
        if b >= t {
            add += a;
        } else {
            for i in (c..=m).rev() {
                let cand = ans[i - c] + a;
                ans[i].maxim(cand);
            }
        }
    }
    out.print_line(ans[m] + add);
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
