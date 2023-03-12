//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModInt9;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_int_vec(n);

    a.sort();
    type Mod = ModInt9;
    let c = Combinations::<Mod>::new(n + 1);
    //    if n % 2 == 0 {
    let mut ans = Mod::one();
    let mut eq = 0;
    let mut rem = n / 2;
    for i in 0..(n + 1) / 2 {
        if a[i] + a[n - i - 1] != a[0] + a[n - 1] {
            ans = Mod::zero();
            break;
        }
        if a[i] != a[n - i - 1] {
            ans *= Mod::new(2);
        }
        if i == 0 || a[i] != a[i - 1] {
            ans *= c.c(rem, eq);
            rem -= eq;
            eq = 0;
        }
        eq += 1;
    }
    // for &i in qty.values() {
    //     ans *= c.c(rem, i);
    //     rem -= i;
    // }
    out_line!(ans);
    return;
    /*}
    let mut left = vec![Mod::zero(); n / 2 + 1];
    let mut right = vec![Mod::zero(); n / 2 + 1];
    left[n / 2] = Mod::one();
    let mut eq = 0;
    let mut last = Mod::one();
    for i in (0..n / 2).rev() {
        if a[i] + a[n - 2 - i] != a[n / 2] + a[n / 2 - 1] {
            break;
        }
        eq += 1;
        if i == 0 || a[i] != a[i - 1] {
            last *= c.c(n / 2 - i, eq);
            eq = 0;
            left[i] = last;
        }
    }
    eq = 0;
    last = Mod::one();
    right[n / 2] = Mod::one();
    for i in (0..n / 2).rev() {
        if a[i + 1] + a[n - 1 - i] != a[n / 2 + 1] + a[n / 2] {
            break;
        }
        eq += 1;
        if i == 0 || a[i + 2] != a[i + 1] {
            last *= c.c(n / 2 - i, eq);
            eq = 0;
            right[i] = last;
        }
    }
    let mut ans = Mod::zero();
    let mut cur = Mod::one();
    let mut eq = 0;
    for i in 0..n / 2 {
        if i == 0 || a[i] != a[i - 1] {
            cur *= c.c(i, eq);
            eq = 0;
            if i == 0 || a[0] + a[n - 1] == a[i + 1] + a[n - i - 1] {
                ans += cur * right[i];
            }
            if i == 0 || a[0] + a[n - 1] == a[i] + a[n - i - 2] {
                ans += cur * left[i];
            }
        }
        eq += 1;
        if a[i] + a[n - i - 1] != a[0] + a[n - 1] {
            cur = Mod::zero();
            break;
        }
    }
    if n == 1 || a[n / 2] != a[n / 2 - 1] {
        ans += cur * c.c(n / 2, eq);
    }
    out_line!(ans);*/
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
