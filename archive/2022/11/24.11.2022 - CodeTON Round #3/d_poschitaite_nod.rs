//{"name":"D. Посчитайте НОД","group":"Codeforces - CodeTON Round 3 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1750/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 5\n4 2 1\n2 1\n1 1\n5 50\n2 3 5 2 3\n4 1000000000\n60 30 1 1\n2 1000000000\n1000000000 2\n","output":"3\n1\n0\n595458194\n200000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPoschitaiteNOD"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::primes::primes_divisors;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_usize_vec(n);

    type Mod = ModIntF;
    let mut ans = Mod::one();
    for (&x, &y) in a.consecutive_iter() {
        if x % y != 0 {
            ans = Mod::zero();
            break;
        }
        let p = x / y;
        let d = primes_divisors(p.into_i64());
        let mut cur = Mod::zero();
        for i in 0i32..(1 << d.len()) {
            let mut c = y;
            for j in 0..d.len() {
                if i.is_set(j) {
                    c *= d[j].0.into_usize();
                }
            }
            let c = Mod::from_index(m / c);
            if i.count_ones() % 2 == 0 {
                cur += c;
            } else {
                cur -= c;
            }
        }
        ans *= cur;
    }
    out_line!(ans);
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
