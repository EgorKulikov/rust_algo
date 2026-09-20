use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::debug;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization3;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let b = input.read_size();
    let m = input.read_size();
    let s = input.read_str();
    drop(input);

    let mut ans = s.clone();
    let mut mem = Memoization3::new(|mem, pos: usize, from: usize, rem: usize| -> usize {
        if pos == n - 1 {
            return 0;
        }
        let add = match m {
            1 => 0,
            2 => pos - from + 1,
            3 => {
                (pos - from + 1) * (pos - from) * 2
                    + from * (pos - from + 1)
                    + (n - 2 - pos) * (pos - from + 1)
            }
            _ => unreachable!(),
        };
        if s[pos] == b'C' {
            return mem.call(pos + 1, from, rem) + add;
        }
        let mut res = mem.call(pos + 1, pos + 1, rem);
        if rem > 0 {
            res.maxim(mem.call(pos + 1, from, rem - 1) + add);
        }
        res
    });
    let mut val = mem.call(0, 0, b);
    debug!(val);
    let mut rem = b;
    let mut from = 0;
    for i in 0..n - 1 {
        if s[i] == b'C' {
            val = mem.call(i + 1, from, rem);
        } else if val != mem.call(i + 1, i + 1, rem) {
            ans[i] = b'C';
            rem -= 1;
            val = mem.call(i + 1, from, rem);
        } else {
            from = i + 1;
        }
    }
    out.print_line(ans);
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
