//{"name":"A - Banned X 2","group":"AtCoder - AtCoder Regular Contest 211 (Div. 2)","url":"https://atcoder.jp/contests/arc211/tasks/arc211_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n0 0 0 1 0 1 0 0 0\n2 1 1 0 0 0 0 0 0\n","output":"1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_size_vec(9);

    let sum = a.copy_sum();
    if a[4] > 0 {
        if 2 * a[4] > sum {
            out.print_line(2 * a[4] - sum - 1);
        } else {
            out.print_line(0);
        }
    } else {
        let mut num_pairs = 0;
        let mut num_singles = 0;
        for i in 0..4 {
            if a[i] > 0 && a[8 - i] > 0 {
                num_pairs += 1;
            } else if a[i] > 0 || a[8 - i] > 0 {
                num_singles += 1;
            }
        }
        if num_singles > 0 || num_pairs != 1 {
            out.print_line(0);
        } else {
            out.print_line(1);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
