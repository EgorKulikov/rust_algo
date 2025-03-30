//{"name":"B - Full House 3","group":"AtCoder - UNIQUE VISION Programming Contest 2025 Spring (AtCoder Beginner Contest 398)","url":"https://atcoder.jp/contests/abc398/tasks/abc398_b","interactive":false,"timeLimit":2000,"tests":[{"input":"1 4 1 4 2 1 3\n","output":"Yes\n"},{"input":"11 12 13 10 13 12 11\n","output":"No\n"},{"input":"7 7 7 7 7 7 7\n","output":"No\n"},{"input":"13 13 1 1 7 4 13\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_int_vec(7);

    out.set_bool_output(BoolOutput::YesNo);
    for i in 0..7 {
        let q = a.copy_count(a[i]);
        if q < 3 {
            continue;
        }
        for j in 0..7 {
            if a[i] == a[j] || a.copy_count(a[j]) < 2 {
                continue;
            }
            out.print_line(true);
            return;
        }
    }
    out.print_line(false);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
