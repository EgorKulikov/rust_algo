//{"name":"Distinct Numbers","group":"CodeChef - SEP221A","url":"https://www.codechef.com/SEP221A/problems/DISTNUMS","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1 5\n2 2\n10 1\n","output":"1\n30\n180\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DistinctNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{ModInt, ModInt7};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::primes::divisors;
use algo_lib::{out_line, value};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();
    let k = input.read_usize();

    let d = divisors(n);
    type Mod = ModInt7;
    value!(Val6: i32 = 1_000_000_006);
    type Mod2 = ModInt<i32, Val6>;
    let mut ans = Mod::one();
    let pow = Mod2::new(2).power(k).val().into_usize();

    for (p, q) in d {
        let p = Mod::new_from_wide(p);
        ans *= p.power(q) * (p.power(q * pow - q + 1) - Mod::one()) / (p - Mod::one());
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
