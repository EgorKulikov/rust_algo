//{"name":"D. Rae Taylor and Trees (easy version)","group":"Codeforces - Codeforces Round 1065 (Div. 3)","url":"https://codeforces.com/contest/2171/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"9\n6\n1 3 4 5 2 6\n4\n3 4 1 2\n5\n4 3 5 1 2\n4\n1 2 3 4\n7\n4 3 5 7 6 2 1\n6\n2 4 6 1 3 5\n3\n2 1 3\n4\n2 4 1 3\n6\n4 2 6 5 1 3\n","output":"Yes\nNo\nNo\nYes\nNo\nYes\nYes\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size_vec(n);

    let mut max = 0;
    for i in (1..n).rev() {
        max.maxim(p[i]);
        if max == n - i {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
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
    }
    eprint!("\x1B[0m");
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive | TaskType::RunTwice => true,
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
