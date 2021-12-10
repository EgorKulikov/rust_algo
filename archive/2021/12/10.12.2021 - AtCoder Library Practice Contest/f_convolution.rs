//{"name":"F - Convolution","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_f","interactive":false,"timeLimit":5000,"tests":[{"input":"4 5\n1 2 3 4\n5 6 7 8 9\n","output":"5 16 34 60 70 70 59 36\n"},{"input":"1 1\n10000000\n10000000\n","output":"871938225\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FConvolution"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::prime_fft::PrimeFFT;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let m = input.read();
    type Mod = ModIntF;
    let a = input.read_vec::<Mod>(n);
    let b = input.read_vec::<Mod>(m);

    let mut fft: PrimeFFT<Mod> = PrimeFFT::new();
    let res = fft.multiply(a.as_slice(), b.as_slice());
    out_line!(res);
}

#[test]
fn big() {
    use algo_lib::collections::iter_ext::IterExt;
    use algo_lib::misc::random::random;
    use algo_lib::numbers::mod_int::BaseModInt;
    type Mod = ModIntF;
    let a = (0..100000)
        .map(|_| Mod::new_from_long((random().gen() % (Mod::module() as u64)) as i64))
        .collect_vec();
    let b = (0..100000)
        .map(|_| Mod::new_from_long((random().gen() % (Mod::module() as u64)) as i64))
        .collect_vec();
    let mut fft: PrimeFFT<Mod> = PrimeFFT::new();
    fft.multiply(a.as_slice(), b.as_slice());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
