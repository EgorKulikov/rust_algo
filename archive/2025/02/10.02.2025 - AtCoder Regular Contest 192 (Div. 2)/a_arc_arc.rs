//{"name":"A - ARC Arc","group":"AtCoder - AtCoder Regular Contest 192 (Div. 2)","url":"https://atcoder.jp/contests/arc192/tasks/arc192_a","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n0 1 0 1 1 1 1 0 1 1 1 0\n","output":"Yes\n"},{"input":"3\n0 0 0\n","output":"No\n"},{"input":"29\n1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    out.set_bool_output(BoolOutput::YesNo);
    if n % 4 == 0 {
        out.print_line(true);
        return;
    }
    if a.copy_count(1) == 0 {
        out.print_line(false);
        return;
    }
    if n % 2 == 1 {
        out.print_line(true);
        return;
    }
    out.print_line(
        a.copy_step_by(2).filter(|&x| x == 1).count() != 0
            && a.copy_skip(1).step_by(2).filter(|&x| x == 1).count() != 0,
    );
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

//START MAIN
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
//END MAIN
