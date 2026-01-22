//{"name":"Good Morning!","group":"Kattis","url":"https://open.kattis.com/problems/goodmorning","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n180\n83\n132\n","output":"180\n80\n133\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = Vec<i32>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let k = input.read_int();

    out.print_line(data.copy_map(|x| ((x - k).abs(), x)).min().unwrap().1);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = Vec::new();
    let mut rec = RecursiveFunction3::new(|rec, n: i32, row: i32, col: i32| {
        if n >= 100 {
            return;
        }
        for r in row..=3 {
            for c in col..3 {
                let mut a = r * 3 + c + 1;
                if a == 11 {
                    a = 0;
                }
                let next = n * 10 + a;
                if next != 0 && a < 10 {
                    pre_calc.push(next);
                    rec.call(next, r, c);
                }
            }
        }
    });
    rec.call(0, 0, 0);
    pre_calc.sort();

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
