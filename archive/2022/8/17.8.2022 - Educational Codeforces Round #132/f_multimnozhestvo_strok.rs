//{"name":"F. Мультимножество строк","group":"Codeforces - Educational Codeforces Round 132 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1709/problem/F","interactive":false,"timeLimit":6000,"tests":[{"input":"1 42 2\n","output":"3\n"},{"input":"2 37 13\n","output":"36871576\n"},{"input":"4 1252 325\n","output":"861735572\n"},{"input":"6 153 23699\n","output":"0\n"},{"input":"15 200000 198756\n","output":"612404746\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMultimnozhestvoStrok"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::prime_fft::PrimeFFT;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let f = input.read_usize();

    type Mod = ModIntF;
    let mut fft = PrimeFFT::new();
    let mut a = vec![Mod::one(); k + 1];
    let mut b = fft.multiply(&a, &a);
    for _ in 1..n {
        let mut tail = b.iter().skip(k + 1).fold(Mod::zero(), |acc, &x| acc + x);
        for i in (0..=k).rev() {
            a[i] = tail + b[i] * Mod::from_index(k - i + 1);
            tail += b[i];
        }
        b = fft.multiply(&a, &a);
    }
    if f < b.len() {
        out_line!(b[f]);
    } else {
        out_line!(0);
    }
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
