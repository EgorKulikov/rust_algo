//{"name":"C - Choosing Souvenirs","group":"AtCoder - AtCoder Weekday Contest 0016 Beta","url":"https://atcoder.jp/contests/awc0016/tasks/awc0016_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 100 500 3\n150 5\n200 8\n50 10\n150 4\n600 9\n","output":"1\n"},{"input":"4 300 500 7\n100 10\n400 3\n600 8\n350 5\n","output":"-1\n"},{"input":"10 200 800 5\n500 3\n1000 10\n250 9\n300 6\n150 8\n800 5\n250 9\n400 7\n900 12\n600 4\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp::Reverse;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_int();
    let r = input.read_int();
    let t = input.read_int();
    let ps = input.read_int_pair_vec(n);

    let mut ans = None;
    for (i, (p, s)) in ps.iter_enumerate() {
        if p >= l && p <= r && s >= t {
            ans.minim((p, Reverse(s), i));
        }
    }
    out.print_line(ans.map(|x| x.2 + 1));
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
