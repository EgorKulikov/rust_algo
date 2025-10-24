//{"name":"Average value - 1","group":"Eolymp - Basecamp - Educational Round #2","url":"https://basecamp.eolymp.com/en/compete/b0f0h1n10h679euo6gn2c7821o/problem/1","interactive":false,"timeLimit":1000,"tests":[{"input":"40   23 27\n  59 68 23    84   27\n53 46\n","output":"46\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let w = input.iter_size().collect::<Vec<_>>();

    if w.is_empty() {
        out.print_line(0);
        return;
    }
    let total = w.copy_sum();
    let min = w.copy_min();
    let max = w.copy_max();
    let count_min = w.copy_count(min);
    let count_max = w.copy_count(max);
    if count_min + count_max >= w.len() {
        out.print_line(0);
        return;
    }
    let total = total - min * count_min - max * count_max;
    let count = w.len() - count_min - count_max;
    out.print_line((total + count / 2) / count);
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
