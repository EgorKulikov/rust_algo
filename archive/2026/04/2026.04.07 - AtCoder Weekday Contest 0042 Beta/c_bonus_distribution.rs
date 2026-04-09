//{"name":"C - Bonus Distribution","group":"AtCoder - AtCoder Weekday Contest 0042 Beta","url":"https://atcoder.jp/contests/awc0042/tasks/awc0042_c","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2\n1 1\n2 2\n","output":"4\n"},{"input":"3 3\n1 2 3\n0 0 0\n","output":"0\n"},{"input":"5 4\n1 2 3 4 5\n10 20 30 40\n","output":"211088321\n"},{"input":"10 10\n3 7 2 5 11 4 8 6 9 1\n100 200 150 300 50 175 225 125 275 80\n","output":"953616835\n"},{"input":"1 1\n1\n5\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_size();
    let p = input.read_long_vec(n);
    let w = input.read_long_vec(d);

    let mut s = p.copy_sum();
    type Mod = ModIntF;
    let mut sw = w.copy_sum();
    let g = gcd(s, sw);
    s /= g;
    sw /= g;
    let mut ans = Mod::new(1);
    for i in 0..n {
        ans *= Mod::from(sw) * p[i] / s;
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
