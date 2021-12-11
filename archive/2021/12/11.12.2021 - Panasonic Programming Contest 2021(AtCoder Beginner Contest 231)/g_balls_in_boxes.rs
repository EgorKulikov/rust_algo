//{"name":"G - Balls in Boxes","group":"AtCoder - Panasonic Programming Contest 2021(AtCoder Beginner Contest 231)","url":"https://atcoder.jp/contests/abc231/tasks/abc231_g","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1\n1 2 3\n","output":"665496245\n"},{"input":"2 2\n1 2\n","output":"499122182\n"},{"input":"10 1000000000\n998244350 998244351 998244352 998244353 998244354 998244355 998244356 998244357 998244358 998244359\n","output":"138512322\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GBallsInBoxes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModIntF};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let k: i32 = input.read();
    let a: Vec<i32> = input.read_vec(n);

    type Mod = ModIntF;
    let mut products = vec![Mod::zero(); n + 1];
    products[0] = Mod::one();
    for i in a {
        for j in (0..n).rev() {
            let p = products[j];
            products[j + 1] += p * Mod::new(i);
        }
    }
    let mut c = Mod::one();
    let mut ans = Mod::zero();
    let mut f = Mod::one();
    let mut p = Mod::one();
    for i in 0..=(n.min(k as usize)) {
        if i != 0 {
            c *= Mod::new(k - (i as i32) + 1);
            c /= Mod::new(i as i32);
            f *= Mod::new(i as i32);
            p /= Mod::new(n as i32);
        }
        ans += c * f * p * products[n - i];
    }
    // println!("{:#?}", ans);
    out_line!(ans);
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
