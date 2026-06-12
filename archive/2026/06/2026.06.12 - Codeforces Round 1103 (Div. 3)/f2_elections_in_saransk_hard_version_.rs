use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::primes::sieve::divisor_table;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = Vec<usize>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, dt: &mut PreCalc) {
    let n = input.read_size();
    let mut x = input.read_size();
    let a = input.read_size_vec(n);

    let mut qty = DefaultHashMap::new(Vec::new());
    for mut i in a {
        while i != 1 {
            let d = dt[i];
            let mut q = 0;
            while i % d == 0 {
                i /= d;
                q += 1;
            }
            qty[d].push(q);
        }
    }
    type Mod = ModInt7;
    let mut ans = Mod::new(1);
    for (k, v) in qty.into_iter() {
        let mut q = 0;
        while x % k == 0 {
            x /= k;
            q += 1;
        }
        if q == 0 {
            ans *= v.copy_sum() + 1;
            continue;
        }
        let max = v.copy_max();
        let mut seen = vec![Mod::zero(); q + 1];
        let mut not_seen = vec![Mod::zero(); q + 1];
        let mut next_seen = vec![Mod::zero(); q + 1];
        let mut next_not_seen = vec![Mod::zero(); q + 1];
        let mut total = Mod::zero();
        for p in 1..=max {
            seen.fill(Mod::zero());
            not_seen.fill(Mod::zero());
            not_seen[0] = Mod::one();
            for mut c in v.copy_iter() {
                c.minim(p);
                next_seen.fill(Mod::zero());
                next_not_seen.fill(Mod::zero());
                for j in 0..=c {
                    for k in 0..=q {
                        if j == p {
                            next_seen[k] += not_seen[k];
                        } else if j + k <= q {
                            next_not_seen[j + k] += not_seen[k];
                        }
                        if j + k <= q {
                            next_seen[j + k] += seen[k];
                        }
                    }
                }
                swap(&mut seen, &mut next_seen);
                swap(&mut not_seen, &mut next_not_seen);
            }
            total += seen[q];
        }
        ans *= total;
    }
    if x != 1 {
        out.print_line(0);
        return;
    }
    out.print_line(ans);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = divisor_table(500_001);

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
