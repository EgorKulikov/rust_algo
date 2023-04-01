//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"m"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::prime_fft::PrimeFFT;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let x = input.read_size() - 1;

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(2 * n + 1);
    let mut catalan = Vec::with_capacity(n + 1);
    for i in 0..=n {
        catalan.push(c.c(2 * i, i) / Mod::from_index(i + 1));
    }
    let mut ans = vec![catalan[n]; n];
    let cat_row = |x: usize| -> Vec<Mod> {
        let mut left = Vec::with_capacity(x + 1);
        for i in (0..=x).rev() {
            left.push(
                c.c(x + i, x) * Mod::from_index(x - i + 1) / Mod::from_index(x + 1) / c.fact(x - i),
            );
        }
        left
    };
    let left = cat_row(x);
    let right = cat_row(n - x - 1);
    let mut fft = PrimeFFT::<Mod>::new();
    let mut res = fft.multiply(&left, &right);
    for i in 0..n {
        res[i] *= c.fact(i);
    }
    let mut delta = Mod::zero();
    for i in (0..n).rev() {
        ans[i] -= delta;
        delta += res[i];
    }
    let left = catalan.iter().take(x + 1).copied().collect_vec();
    let right = catalan.iter().take(n - x).copied().collect_vec();
    let res2 = fft.multiply(&left, &right);
    delta = Mod::zero();
    for i in 0..n {
        ans[i] -= delta;
        delta += res2[n - i - 1] * catalan[i];
    }
    output().print_per_line(&ans);
}

#[test]
fn test() {
    use algo_lib::collections::arr2d::Arr2d;
    use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
    let catalan = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786];
    let mut res = Arr2d::new(10, 10, 0);
    res[(0, 0)] = 1;
    for i in 1..10 {
        for j in 1..10 {
            for k in 0..i {
                res[(i, j)] += res[(k, j - 1)] * catalan[i - k - 1];
            }
        }
    }
    for i in 1..10 {
        for j in 1..10 {
            print!("{:8} ", res[(i, j)]);
        }
        println!();
    }
    /*
    for n in 1..10 {
        let mut qty = Arr2d::new(n, n, 0);
        let mut rec = RecursiveFunction3::new(|f, l, r, d| -> usize {
            if l == r {
                return 1;
            }
            let mut total = 0;
            for i in l..r {
                let cur = f.call(l, i, d + 1) * f.call(i + 1, r, d + 1);
                qty[(i, d)] += cur;
                total += cur;
            }
            total
        });
        rec.call(0, n, 0);
        println!("n = {n}");
        for i in 0..n {
            for j in 0..n {
                print!("{:8} ", qty[(j, i)]);
            }
            println!();
        }
    }*/
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
