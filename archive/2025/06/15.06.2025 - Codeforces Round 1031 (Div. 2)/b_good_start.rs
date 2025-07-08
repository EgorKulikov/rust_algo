//{"name":"B. Good Start","group":"Codeforces - Codeforces Round 1031 (Div. 2)","url":"https://codeforces.com/contest/2113/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n6 5 2 3\n-1 -2 5 4\n4 4 2 2\n0 0 3 1\n10 9 3 2\n0 0 4 3\n10 9 3 2\n0 0 6 3\n5 5 2 2\n-1 -1 4 -1\n5 5 2 2\n-1 -1 2 3\n7 8 2 4\n0 0 0 5\n","output":"Yes\nNo\nNo\nYes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _w = input.read_long();
    let _h = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let x1 = input.read_long();
    let y1 = input.read_long();
    let x2 = input.read_long();
    let y2 = input.read_long();

    if x1 == x2 {
        out.print_line((y1 - y2) % b == 0);
        return;
    }
    if y1 == y2 {
        out.print_line((x1 - x2) % a == 0);
        return;
    }
    out.print_line((x1 - x2) % a == 0 || (y1 - y2) % b == 0);
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
