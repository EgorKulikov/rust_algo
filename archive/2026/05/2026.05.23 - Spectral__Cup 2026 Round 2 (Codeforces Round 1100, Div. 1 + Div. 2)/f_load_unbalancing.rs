use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_2d::Memoization2d;
use algo_lib::misc::recursive_function::{Callable, Callable2, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::{BitIter, BitOps};
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n).sorted();

    let sum = Vec::with_gen(1 << (n - 1), |i| {
        let mut cur = 0;
        for j in 0..n - 1 {
            if i.is_set(j) {
                cur += a[j];
            }
        }
        cur
    });
    let mut mem = Memoization2d::new(1 << (n - 1), k + 1, |mem, mask, qty| -> i64 {
        if qty == 1 {
            sum[mask]
        } else if (mask.count_ones() as usize) < qty {
            0
        } else {
            let mut res = 0;
            let q1 = qty / 2;
            let q2 = qty - q1;
            for i in BitIter::new(mask.without_bit(mask.lowest_bit())) {
                if i == 0 {
                    continue;
                }
                res.maxim(mem.call(i, q1).min(mem.call(mask ^ i, q2)));
            }
            res
        }
    });
    let mut val = vec![Vec::new(); k + 1];
    let mut get = RecursiveFunction::new(|rec, qty: usize| -> Vec<i64> {
        if val[qty].is_empty() {
            if qty == 1 {
                val[qty] = sum.clone();
            } else {
                let q1 = qty / 2;
                let q2 = qty - q1;
                let a = rec.call(q1);
                let b = rec.call(q2);
                for mask in usize::iter_all(n - 1) {
                    if (mask.count_ones() as usize) < qty {
                        val[qty].push(0);
                        continue;
                    }
                    let mut res = 0;
                    for i in BitIter::new(mask.without_bit(mask.lowest_bit())) {
                        res.maxim(a[i].min(b[mask ^ i]));
                    }
                    val[qty].push(res);
                }
            }
        }
        val[qty].clone()
    });
    // out.print_line(mem.call(usize::all_bits(n - 1), k) + a[n - 1]);
    out.print_line(get.call(k)[usize::all_bits(n - 1)] + a[n - 1]);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
