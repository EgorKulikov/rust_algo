use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::integer_sqrt::IntegerSqrt;
use algo_lib::numbers::num_utils::UpperDiv;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let cr = input.read_size();
    let cs = input.read_size();
    drop(input);

    let mut mem = Memoization::new(|mem, n: usize| -> (usize, usize) {
        if n == 1 {
            (0, 0)
        } else {
            let mut res = usize::MAX;
            let mut best_i = 0;
            let sq = n.lower_sqrt();
            for i in 1..=sq {
                if res.minim(cr * i + cs + mem.call(n.upper_div(i + 1)).0) {
                    best_i = i;
                }
            }
            for j in 0..=sq {
                let i = n.upper_div(j + 1) - 1;
                if i == 0 {
                    continue;
                }
                if res.minim(cr * i + cs + mem.call(n.upper_div(i + 1)).0) {
                    best_i = i;
                }
            }
            (res, best_i)
        }
    });
    let (ans, p) = mem.call(n);
    out.print_line(ans);
    let step = n.upper_div(p + 1);
    out.print_line(p);
    out.print_line_iter((1..=p).map(|i| i * step));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;
pub static TEST_TYPE: TestType = TestType::MultiNumber;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    eprint!("\x1B[0m");
    output.flush();
    is_exhausted
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
