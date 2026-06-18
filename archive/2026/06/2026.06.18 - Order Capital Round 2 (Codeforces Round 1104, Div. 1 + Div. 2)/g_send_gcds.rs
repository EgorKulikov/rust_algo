use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::lcm;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::primes::sieve::{divisor_table, primes};
use algo_lib::string::str::StrReader;
use std::io::Write;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let good = get_good();
    let p = primes(864);

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let a = input.read_size_vec(n);

        let mut len = 0;
        let mut val = 0;
        let mut b = Vec::new();
        for i in 0..150 {
            let mut x = 1;
            while x * p[i] <= 1_000_000 {
                x *= p[i];
            }
            b.push(x);
        }
        for i in a {
            len += 20;
            val <<= 20;
            val += i;
            while len >= 18 {
                len -= 18;
                b.push(good[val >> len]);
                val &= usize::all_bits(len);
            }
        }
        if len > 0 {
            val <<= 18 - len;
            b.push(good[val]);
        }
        out.print_line(b.len());
        out.print_line(b);
        out.flush();
    }
}

fn get_good() -> Vec<usize> {
    let p = primes(864);
    let dt = divisor_table(1_000_001);
    let mut good = Vec::new();
    for i in 1..=1_000_000 {
        let mut j = i;
        let mut is_good = true;
        while j != 1 {
            let pp = dt[j];
            if pp > p[149] {
                is_good = false;
                break;
            }
            j /= pp;
        }
        if is_good {
            good.push(i);
        }
    }
    good
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let good = get_good();
    let mut id = vec![usize::MAX; 1_000_001];
    for (i, g) in good.iter_enumerate() {
        id[g] = i;
    }

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_size();
        let k = input.read_size();

        let b = Vec::with_gen(k - 150, |i| {
            let mut cur = 1;
            for j in 0..150 {
                writeln!(out, "? {} {}", j + 1, i + 151).unwrap();
                out.flush();
                let v = input.read_size();
                cur = lcm(cur, v);
            }
            cur
        });
        let mut len = 0;
        let mut val = 0;
        let mut a = Vec::new();
        for i in b {
            len += 18;
            val <<= 18;
            val += id[i];
            while len >= 20 {
                len -= 20;
                a.push(val >> len);
                val &= usize::all_bits(len);
            }
        }
        assert_eq!(a.len(), n);
        out.print_line(('!', a));
        out.flush();
    }
}

pub static TEST_TYPE: TestType = TestType::RunTwiceSingle;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
