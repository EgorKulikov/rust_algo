//{"name":"C - Watering the Flower Bed","group":"AtCoder - AtCoder Weekday Contest 0034 Beta","url":"https://atcoder.jp/contests/awc0034/tasks/awc0034_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 10 100\n8 7 9 12 11\n","output":"300\n"},{"input":"8 2 15 50\n12 10 14 13 11 15 9 16\n","output":"750\n"},{"input":"10 4 20 1000\n18 15 12 19 17 14 16 20 13 21\n","output":"15000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let t = input.read_long();
    let c = input.read_long();
    let a = input.read_long_vec(n);

    let mut delta = 0;
    let mut rem = vec![0; n];
    let mut ans = 0;
    for i in 0..n {
        delta -= rem[i];
        let cur = a[i] + delta;
        if cur < t {
            ans += (t - cur) * c;
            delta += t - cur;
            if i + k < n {
                rem[i + k] += t - cur;
            }
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
