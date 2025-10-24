//{"name":"Planting Trees","group":"DMOJ - Waterloo 2025 Fall A","url":"https://dmoj.ca/problem/waterloo2025fa","interactive":false,"timeLimit":4000,"tests":[{"input":"3 2\n1 2\n","output":"2 1\n"},{"input":"6 2\n1 3\n","output":"4 2\n"},{"input":"5 2\n1 3\n","output":"0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use std::iter::repeat;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::inverse_factorials;
use algo_lib::numbers::mod_int::prime_fft::PrimeFFT;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::factorial;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let d = input.read_size_vec(m);

    type Mod = ModIntF;
    let mut base = vec![Mod::zero(); n - 1];
    let i_f = inverse_factorials::<u32, Mod>(n - 1);
    for i in d.copy_iter() {
        base[i - 1] = i_f[i - 1];
    }
    let mut fft = PrimeFFT::new();
    let mut rec = RecursiveFunction::new(|rec, exp: usize| -> Vec<Mod> {
        if exp == 1 {
            base.clone()
        } else if exp % 2 == 0 {
            let half = rec.call(exp / 2);
            let mut res = fft.multiply(&half, &half);
            res.truncate(n - 1);
            res
        } else {
            let half = rec.call(exp - 1);
            let mut res = fft.multiply(&half, &base);
            res.truncate(n - 1);
            res
        }
    });
    let res_m1 = rec.call(n - 1);
    let res = fft.multiply(&res_m1, &base);
    if res[n - 2] == Mod::zero() {
        out.print_line_iter(repeat(0).take(m));
        return;
    }
    let den = res[n - 2];
    eprintln!("total = {}", den * factorial::<Mod>(n - 2));
    let mut ans = Vec::with_capacity(m);
    for i in 0..m {
        ans.push(res_m1[n - 2 - (d[i] - 1)] * n / den * i_f[d[i] - 1]);
    }
    out.print_line(ans);
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
