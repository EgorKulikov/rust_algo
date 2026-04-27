//{"name":"D - Card Taking Game","group":"AtCoder - AtCoder Weekday Contest 0054 Beta","url":"https://atcoder.jp/contests/awc0054/tasks/awc0054_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2 3 4\n","output":"6 4\n"},{"input":"5\n3 -1 2 -5 4\n","output":"-2 5\n"},{"input":"8\n10 -3 7 2 -8 5 1 6\n","output":"14 6\n"},{"input":"20\n15 -7 23 -4 8 19 -12 6 3 -1 14 -9 7 25 -3 11 2 -18 20 5\n","output":"77 27\n"},{"input":"1\n-1000000000\n","output":"-1000000000 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::Callable2;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut mem = Memoization2d::new(n + 1, n + 1, |mem, l, r| -> i64 {
        if l == r {
            0
        } else {
            (a[l] - mem.call(l + 1, r)).max(a[r - 1] - mem.call(l, r - 1))
        }
    });
    let delta = mem.call(0, n);
    let sum = a.copy_sum();
    out.print_line(((sum + delta) / 2, (sum - delta) / 2));
}

pub static TEST_TYPE: TestType = TestType::Single;
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
