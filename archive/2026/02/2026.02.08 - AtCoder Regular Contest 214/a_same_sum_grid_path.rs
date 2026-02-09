//{"name":"A - Same Sum Grid Path","group":"AtCoder - AtCoder Regular Contest 214","url":"https://atcoder.jp/contests/arc214/tasks/arc214_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n101\n0??\n1??\n","output":"101\n010\n101\n"},{"input":"4\n2026\n?2?8\n????\n?214\n","output":"-1\n"},{"input":"2\n99\n99\n","output":"99\n99\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut s = input.read_char_table(n, n);

    for i in 0..=2 * n - 2 {
        let mut val = None;
        for j in 0..n {
            if j <= i && i - j < n {
                let c = s[(j, i - j)];
                if c != b'?' {
                    if val.is_none() {
                        val = Some(c);
                    } else if val != Some(c) {
                        out.print_line(-1);
                        return;
                    }
                }
            }
        }
        let val = val.unwrap_or(b'0');
        for j in 0..n {
            if j <= i && i - j < n {
                s[(j, i - j)] = val;
            }
        }
    }
    out.print_table(&s);
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
