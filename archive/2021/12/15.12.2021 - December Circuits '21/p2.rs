//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/algorithm/board-beauty-360c5ba7/","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1\n2\n","output":"1\n555555562\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt7};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    type Mod = ModInt7;
    let mut qty = vec![Mod::zero(); 2 * n + 1];
    for i in 2..=(2 * n) {
        for j in (i..=(2 * n)).step_by(i) {
            qty[i] += Mod::new(if j <= n { j - 1 } else { 2 * n + 1 - j } as i32);
        }
    }
    let inv = Mod::new((2 * n - 1) as i32).inv().unwrap();
    let mut ans = Mod::zero();
    for i in qty.iter().cloned() {
        ans += i;
    }
    ans *= Mod::new(2);
    ans *= inv;
    let mut divs = vec![Mod::zero(); 2 * n + 1];
    let mut ways = vec![Mod::zero(); 2 * n + 1];
    for i in 2..=(2 * n) {
        for j in (i..=(2 * n)).step_by(i) {
            divs[j] += inv;
        }
        ways[i] += divs[i] * divs[i];
        ans -= ways[i] * qty[i];
        for j in ((2 * i)..=(2 * n)).step_by(i) {
            let val = ways[i];
            ways[j] -= val;
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
