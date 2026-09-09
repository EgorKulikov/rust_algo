use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::multiplicative_function::MulitplicativeFunction;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::primes::factorize::all_divisors;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    type Mod = ModIntF;
    let d = all_divisors(n + 1, true);
    let mut sets = vec![Vec::new(); n + 1];
    let mu = MulitplicativeFunction::mobius().calculate_up_to(n + 1);
    for i in 2..=n {
        for j in d[i].copy_iter() {
            if j != 1 && mu[j] != 0 {
                sets[i].push(j);
            }
        }
    }
    let mut mult = DefaultHashMap::new(Mod::one());
    let mut special = DefaultHashMap::new(0);
    let mut mu_val = FxHashMap::default();
    for i in 2..=n {
        for j in sets[i].copy_iter() {
            special[j] += 1;
            mu_val.insert(j, mu[j]);
        }
    }
    for i in 2..n {
        for x in sets[i].copy_iter() {
            for y in sets[i + 1].copy_iter() {
                mult[x * y] *= 2;
                mu_val.insert(x * y, mu[x] * mu[y]);
            }
        }
    }
    for i in (3..n).step_by(2) {
        for j in sets[i].copy_iter() {
            mult[2 * j] /= 4;
            mult[2 * j] *= 3;
        }
    }
    let mut ans = Mod::zero();
    let c = Combinations::<Mod>::new(n + 1);
    for key in mu_val.keys() {
        let val = c.c(n, special[key]) * c.fact(special[key]) * mult[key] - 1;
        ans -= val * mu_val[key];
    }
    out.print_line(ans + 1);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::Single;
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
