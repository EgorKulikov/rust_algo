//{"name":"C - Bargain Shopping Mall Tour","group":"AtCoder - AtCoder Weekday Contest 0021 Beta","url":"https://atcoder.jp/contests/awc0021/tasks/awc0021_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n500 3 200 300 100\n300 2 400 150\n800 4 250 250 250 250\n","output":"450\n"},{"input":"4 3\n100 2 50 60\n200 3 300 100 50\n150 1 100\n80 2 90 85\n","output":"355\n"},{"input":"5 2\n1000000000 5 500000000 400000000 300000000 200000000 100000000\n100 3 50 40 30\n999999999 2 1000000000 1000000000\n500 4 200 150 100 80\n10000 10 5000 4000 3000 2000 1000 900 800 700 600 500\n","output":"1500000001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut p = Vec::new();
    for _ in 0..n {
        let c = input.read_long();
        let m = input.read_size();
        let pp = input.read_long_vec(m);

        let sum = pp.copy_sum();
        if sum > c {
            p.push(sum - c);
        }
    }
    p.sort();
    p.reverse();
    out.print_line(p.into_iter().take(k).sum::<i64>());
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
