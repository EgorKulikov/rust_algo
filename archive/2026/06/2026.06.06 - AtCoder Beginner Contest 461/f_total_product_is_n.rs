use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization_3d::Memoization3d;
use algo_lib::misc::recursive_function::Callable3;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::factorials;
use algo_lib::numbers::primes::factorize::Factorize;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();

    let pq = n.prime_divisors();
    let mut total = 0;
    for (_, q) in pq {
        total += q / 2 + 1;
    }
    let d = n.divisors();
    type Mod = ModIntF;
    let f = factorials::<Mod>(total + 2);
    let t = Arr2d::with_gen(d.len(), d.len(), |a, b| {
        if d[a] % d[b] == 0 {
            Some(d.lower_bound(&(d[a] / d[b])))
        } else {
            None
        }
    });
    let mut mem = Memoization3d::new(
        d.len(),
        d.len() + 1,
        total + 2,
        |mem, rem, max, len| -> (Mod, Mod) {
            let mut sum = Mod::zero();
            let mut qty = Mod::zero();
            if rem == 0 {
                qty += f[len];
            }
            for i in 0..max {
                if let Some(next) = t[(rem, i)] {
                    let (call_sum, call_qty) = mem.call(next, i, len + 1);
                    sum += call_sum + call_qty * d[i] as i64;
                    qty += call_qty;
                }
            }
            (sum, qty)
        },
    );
    out.print_line(mem.call(d.len() - 1, d.len(), 0).0);
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
