//{"name":"Fun function","group":"Eolymp - Basecamp - Educational Round #2","url":"https://basecamp.eolymp.com/en/compete/b0f0h1n10h679euo6gn2c7821o/problem/4","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3\n","output":"4\n"},{"input":"34 12\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_size();
    let y = input.read_size();

    let mut mem = Memoization2d::new(x + 1, y + 1, |mem, x, y| {
        if x == 0 || y == 0 {
            0
        } else if x <= y {
            mem.call(x - 1, y.saturating_sub(2)) + mem.call(x.saturating_sub(2), y - 1) + 2
        } else {
            mem.call(x.saturating_sub(2), y.saturating_sub(2)) + 1
        }
    });
    out.print_line(mem.call(x, y));
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
