//{"name":"hh","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut ans = Arr2d::new(4 * n + 1, (4 * n + 2) * 2 - 1, b' ');
    let d1 = ans.d1();
    let d2 = ans.d2();
    for i in 0..d1 {
        ans[(i, 0)] = b'*';
        ans[(i, d2 - 1)] = b'*';
    }
    for i in (0..d2).step_by(2) {
        ans[(0, i)] = b'*';
        ans[(d1 - 1, i)] = b'*';
    }
    for i in 2..(d1 - 2) {
        ans[(i, 2)] = b'*';
        ans[(i, d2 - 3)] = b'*';
    }
    for i in 2..d1 / 2 {
        ans[(i, d2 / 2 - 2 * i + 1)] = b'*';
        ans[(i, d2 / 2 + 3)] = b'*';
    }
    for i in ((d2 / 2 + 3)..d2 - 3).step_by(2) {
        ans[(d1 / 2, i)] = b'*';
    }
    for i in d1 / 2..d1 - 2 {
        ans[(i, d2 / 2 + 2 - 2 * (d1 - i) + 1)] = b'*';
    }
    out.print_table(&ans);
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
