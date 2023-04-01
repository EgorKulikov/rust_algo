//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::time_tracker::TimeTracker;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::number_ext::Power;
use algo_lib::numbers::prime_fft::PrimeFFT;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut tt = TimeTracker::new();
    type Mod = ModIntF;

    let n = input.read_size();
    let a: Mod = input.read();
    let b: Mod = input.read();
    let c: Mod = input.read();

    fn f(x: usize) -> usize {
        x.count_ones().into_usize()
    }
    fn g(mut x: usize) -> usize {
        let mut res = 0;
        while x > 0 {
            res += x % 3;
            x /= 3;
        }
        res
    }

    let b_power = powers(b, 1000);
    let c_power = powers(c, 1000);

    let step2 = 2.power(20);
    let mut left = Vec::with_capacity(step2);
    let mut apow = Mod::one();
    for i in 0..step2 {
        left.push(b_power[f(i)] * apow);
        apow *= a;
    }
    tt.milestone("left gen");
    let step3 = 3.power(12);
    let mut right = Vec::with_capacity(step3);
    for i in 0..step3 {
        right.push(c_power[g(i)]);
    }
    right.reverse();
    tt.milestone("right gen");
    let mut fft = PrimeFFT::new();
    let res = fft.multiply(&left, &right);
    tt.milestone("fft");
    let mut special = Vec::new();
    for i in (0..n).step_by(step2) {
        special.push(i);
    }
    for i in (0..n).step_by(step3) {
        special.push(i);
    }
    special.sort();
    tt.milestone("special gen");
    let mut ans = Mod::zero();
    let mut apow = Mod::one();
    let amul = a.power(step2);
    for (&from, &to) in special.consecutive_iter() {
        if from == to {
            continue;
        }
        let pos = from % step2 + right.len() - 1 - from % step3;
        ans += res[pos] * apow * b_power[f(from / step2)] * c_power[g(from / step3)];
        if to % step2 == 0 {
            apow *= amul;
        }
    }
    tt.milestone("main part");
    let mut apow = a.power(special[special.len() - 1]);
    for i in special[special.len() - 1]..=n {
        ans += apow * b_power[f(i)] * c_power[g(i)];
        apow *= a;
    }
    tt.milestone("tail");
    ans -= Mod::one();
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
}
//END MAIN
