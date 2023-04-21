//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::vec_ext::{Bounds, IncDec};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::time_tracker::TimeTracker;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::primes::Factorize;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut tt = TimeTracker::new();
    tt.disable();
    let n = input.read_size();
    let m = input.read_long();
    let a = input.read_int_vec(n).inc_by_one();

    let pd = m.prime_divisors();
    let max_p = pd.iter().map(|&(_, e)| e).sum::<usize>();
    let mut d = m.divisors();
    d.sort();
    let k = d.len();
    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(k + n + 1);
    let mut ways = Arr2d::new(max_p + 1, k, Mod::zero());
    tt.milestone("prep");
    let go = Arr2d::generate(k, k, |i, j| {
        if (m / d[i]) % d[j] == 0 {
            d.bin_search(&(d[i] * d[j]))
        } else {
            None
        }
    });
    tt.milestone("go");
    ways[(0, 0)] = Mod::one();
    for i in 1..k {
        for l in (0..k).rev() {
            if go[(l, i)].is_none() {
                continue;
            }
            for j in (0..=max_p).rev() {
                if ways[(j, l)] == Mod::zero() {
                    continue;
                }
                let mut val = go[(l, i)];
                let mut sh = 1;
                let mut mul = Mod::new_from_wide(d[i] - 1);
                while val.is_some() {
                    let add = c.c(j + sh, sh) * ways[(j, l)] * mul;
                    ways[(j + sh, val.unwrap())] += add;
                    sh += 1;
                    val = go[(val.unwrap(), i)];
                    mul *= Mod::new_from_wide(d[i] - 1);
                }
            }
        }
    }
    tt.milestone("ways");
    let mut cur = vec![Mod::zero(); max_p + 1];
    cur[0] = Mod::one();
    for i in a {
        for j in (1..=max_p).rev() {
            cur[j] = cur[j] * Mod::new(i) + cur[j - 1];
        }
        cur[0] *= Mod::new(i);
    }
    tt.milestone("cur");
    let mut ans = Mod::zero();
    for i in 0..=max_p {
        if i > n {
            break;
        }
        if i == n {
            ans += ways[(i, k - 1)];
            continue;
        }
        for j in 0..k {
            let mut other_ways = Mod::one();
            let mut rem = m / d[j];
            for &(p, _) in &pd {
                let mut cur = 0;
                while rem % p == 0 {
                    cur += 1;
                    rem /= p;
                }
                other_ways *= c.c(n - i + cur - 1, n - i - 1);
            }
            ans += other_ways * ways[(i, j)] * cur[i];
        }
    }
    tt.milestone("ans");

    out_line!(ans);
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
    stress_test::stress_test(run, tester::check);
}
//END MAIN
