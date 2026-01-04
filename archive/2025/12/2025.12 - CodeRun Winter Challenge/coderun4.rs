//{"name":"coderun4","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::multiplicative_function::MulitplicativeFunction;
use algo_lib::numbers::primes::sieve::primes;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let p = primes(100);
    let mut cand = BinaryHeap::new();
    cand.push(Reverse(1usize));
    let nd = MulitplicativeFunction::divisor_count();
    let mut ans = Vec::new();
    let mut last = 0;
    while let Some(cur) = cand.pop() {
        let q = nd.call(cur.0 as u64);
        if last.maxim(q) {
            ans.push(cur.0);
            for i in p.copy_iter() {
                if usize::MAX / i >= cur.0 {
                    cand.push(Reverse(cur.0 * i));
                }
            }
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let l = input.read_size();
        let r = input.read_size();
        out.print_line(ans.upper_bound(&r) - ans.lower_bound(&l));
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
