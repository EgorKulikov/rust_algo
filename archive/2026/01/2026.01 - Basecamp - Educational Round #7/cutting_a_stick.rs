//{"name":"Cutting a stick","group":"Eolymp - Basecamp - Educational Round #7","url":"https://eolymp.com/en/compete/0fdopgb6et7g3allmrhbicn9cg/problem/8","interactive":false,"timeLimit":1000,"tests":[{"input":"100\n3\n25 50 75\n10\n4\n4 5 7 8\n0\n","output":"The minimum cutting is 200.\nThe minimum cutting is 22.\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::input_iter::InputIterable;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::io::Write;
use std::iter::once;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_int();
    if l == 0 {
        return;
    }
    let n = input.read_size();
    let c = once(0)
        .chain(input.iter_int().take(n))
        .chain(once(l))
        .collect::<Vec<_>>();

    let mut mem = Memoization2d::new(n + 2, n + 2, |mem, l, r| {
        if l + 1 == r {
            0
        } else {
            let mut res = i32::MAX;
            for i in l + 1..r {
                res.minim(c[r] - c[l] + mem.call(l, i) + mem.call(i, r));
            }
            res
        }
    });
    writeln!(out, "The minimum cutting is {}.", mem.call(0, n + 1)).unwrap();
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
