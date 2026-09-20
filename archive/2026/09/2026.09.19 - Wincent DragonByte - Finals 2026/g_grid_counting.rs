use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_utils::factorial;
use algo_lib::numbers::rational::Rational;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let rl = if n == 9 { 3 } else { 2 };
    let cl = match n {
        4 => 2,
        8 => 4,
        _ => 3,
    };
    let mut r = vec![0; n];
    let mut c = vec![0; n];
    let mut s = Arr2d::new(n / rl, n / cl, 0);
    for i in 0..n {
        c[i].set_bit(i);
        s[(0, i / cl)].set_bit(i);
    }
    let mut total = 0;
    let mut good = 0;
    let f = factorial::<usize>(n);
    let gq = vec![f * 4 / n, f * 3 / n, f * 2 / n];
    let mut rec = RecursiveFunction3::new(|rec, rr: usize, cc: usize, q_same: usize| {
        if rr == n {
            total += f;
            good += gq[q_same];
            return;
        }
        if cc == n {
            rec.call(rr + 1, 0, q_same);
            return;
        }
        for i in 0..n {
            if r[rr].is_set(i) || c[cc].is_set(i) || s[(rr / rl, cc / cl)].is_set(i) {
                continue;
            }
            r[rr].set_bit(i);
            c[cc].set_bit(i);
            s[(rr / rl, cc / cl)].set_bit(i);
            let q_same =
                if rr == n - 1 && cc == 0 && i == n - 1 || rr == n - 1 && cc == n - 1 && i == 0 {
                    q_same + 1
                } else {
                    q_same
                };
            rec.call(rr, cc + 1, q_same);
            r[rr].unset_bit(i);
            c[cc].unset_bit(i);
            s[(rr / rl, cc / cl)].unset_bit(i);
        }
    });
    rec.call(1, 0, 0);
    out.print_line(good);
    out.print_line(Rational::new(good as i64, total as i64));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;
pub static TEST_TYPE: TestType = TestType::Single;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    // let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    solve(&mut input, &mut output, 1, &pre_calc);
    eprint!("\x1B[0m");
    output.flush();
    input.is_exhausted()
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
