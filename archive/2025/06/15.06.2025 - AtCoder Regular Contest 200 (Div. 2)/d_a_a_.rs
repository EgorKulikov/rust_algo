//{"name":"D - |A + A|","group":"AtCoder - AtCoder Regular Contest 200 (Div. 2)","url":"https://atcoder.jp/contests/arc200/tasks/arc200_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6 5\n3 2\n8 3\n","output":"Yes\n3\n3 1 4\nNo\nYes\n2\n0 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use std::iter::once;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_size();
    let k = input.read_size();

    if k % 2 == 1 {
        out.print_line(true);
        out.print_line((k + 1) / 2);
        out.print_line_iter(0..=k / 2);
        return;
    }
    if m % k == 0 {
        out.print_line(true);
        out.print_line(k);
        out.print_line_iter((0..m).step_by(m / k));
        return;
    }
    if k >= 6 {
        out.print_line(true);
        out.print_line(k / 2);
        out.print_line_iter((0..k / 2 - 1).chain(once(k / 2)));
        return;
    }
    out.print_line(false);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
