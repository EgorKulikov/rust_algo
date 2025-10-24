//{"name":"Infinite Sequence - 2","group":"Eolymp - Basecamp - Educational Round 1","url":"https://basecamp.eolymp.com/en/compete/1v399r4bst3f1apjnuj8as5pbc/problem/5","interactive":false,"timeLimit":2000,"tests":[{"input":"12 2 3 1 0\n","output":"8\n"},{"input":"10000000 2 3 10000000 10000000\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size();
    let q = input.read_size();
    let x = input.read_size();
    let y = input.read_size();

    let mut mem = Memoization1d::new(1_000_000, |mem, n| -> i64 {
        if n == 0 {
            1
        } else {
            mem.call((n / p).saturating_sub(x)) + mem.call((n / q).saturating_sub(y))
        }
    });
    let mut rec = RecursiveFunction::new(|rec, n: usize| {
        if n < 1_000_000 {
            mem.call(n)
        } else {
            rec.call((n / p).saturating_sub(x)) + rec.call((n / q).saturating_sub(y))
        }
    });
    out.print_line(rec.call(n));
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
