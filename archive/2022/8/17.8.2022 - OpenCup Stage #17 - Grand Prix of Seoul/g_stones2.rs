//{"name":"G. Stones 2","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/G/","interactive":false,"timeLimit":4000,"tests":[{"input":"4\nWBWB\n6 4 5 3\n","output":"72\n"},{"input":"8\nWBBWBWBB\n6 4 8 2 5 3 1 5\n","output":"218304\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GStones2"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::factorial;
use algo_lib::numbers::prime_fft::PrimeFFT;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Str = input.read();
    let a = input.read_int_vec(n);

    type Mod = ModIntF;
    let mut fft: PrimeFFT<Mod> = PrimeFFT::new();

    let mut ans = Mod::zero();
    for c in [b'W', b'B'] {
        let mut sums = Vec::with_capacity(n + 1);
        let mut c_sum = Mod::zero();
        for i in 0..n {
            if s[i] != c {
                c_sum += Mod::new(a[i]);
            }
            sums.push(c_sum);
        }
        let mut x = Vec::with_capacity(n);
        for i in 0..n {
            if s[i] == c {
                x.push(sums[i]);
            } else {
                x.push(Mod::zero());
            }
        }
        let mut y = Vec::with_capacity(n);
        for i in (0..n).rev() {
            if s[i] == c {
                y.push(Mod::one());
            } else {
                y.push(Mod::zero());
            }
        }
        let r = fft.multiply(&x, &y);
        let r1 = r.iter().skip(n - 1).cloned().collect_vec();
        let r2 = r.into_iter().rev().skip(n - 1).collect_vec();
        for i in 2..n {
            ans += (r1[i] - r2[i])
                / (Mod::from_index(i - 1) * Mod::from_index(i) * Mod::from_index(i + 1));
        }
    }
    ans *= Mod::new(2);
    ans *= factorial(n);
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
