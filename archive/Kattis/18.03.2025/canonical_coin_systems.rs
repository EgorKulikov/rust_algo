//{"name":"Canonical Coin Systems","group":"Kattis","url":"https://open.kattis.com/problems/canonical","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2 4 8\n","output":"canonical\n"},{"input":"3\n1 5 8\n","output":"non-canonical\n"},{"input":"6\n1 5 10 25 100 200\n","output":"canonical\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let c = input.read_size_vec(n);

    let mut mem = Memoization1d::new(c[Back(0)] + c[Back(1)], |mem, v| -> (usize, usize) {
        if v == 0 {
            (0, 0)
        } else {
            let mut min = usize::MAX;
            let mut can = usize::MAX;
            for c in c.copy_iter() {
                if c > v {
                    break;
                }
                let (call_min, call_can) = mem.call(v - c);
                min.minim(call_min + 1);
                can = call_can + 1;
            }
            (min, can)
        }
    });
    for i in 1..c[Back(0)] + c[Back(1)] {
        let (min, can) = mem.call(i);
        if min != can {
            out.print_line("non-canonical");
            return;
        }
    }
    out.print_line("canonical");
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
