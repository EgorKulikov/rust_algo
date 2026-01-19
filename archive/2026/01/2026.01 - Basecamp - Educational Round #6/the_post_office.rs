//{"name":"The post office","group":"Eolymp - Basecamp - Educational Round #6","url":"https://eolymp.com/en/compete/ikoi44ho994uj2rcj49h0l45v4/problem/9","interactive":false,"timeLimit":1000,"tests":[{"input":"5 800\n100 200 300 400 500\n","output":"1300\n"},{"input":"5 800\n400 400 400 400 400\n","output":"2000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_vec::Memoization1d;
use algo_lib::misc::recursive_function::Callable;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::{BitIter, BitOps};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let p = input.read_size();
    let m = input.read_size_vec(n);

    let mut sum = vec![0];
    for i in usize::iter_all(n).skip(1) {
        sum.push(sum[i.without_bit(i.highest_bit())] + m[i.highest_bit()]);
    }
    let mut mem = Memoization1d::new(1 << n, |mem, mask| -> usize {
        let mut res = 0;
        for i in BitIter::new(mask) {
            if sum[i] == 1000 {
                res.maxim(1 + mem.call(mask - i));
            }
        }
        res
    });
    out.print_line(sum[Back(0)] - (1000 - p) * mem.call(usize::all_bits(n)));
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
