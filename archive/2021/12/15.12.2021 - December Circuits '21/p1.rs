//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/algorithm/lucky-trip-03c952a0/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1\n0 2 1\n1\n42 30\n4\n4 1 8\n4\n60 28\n2\n1 0 1\n2\n30 30\n","output":"0\n1\n350000003\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::collections::arr3d::Arr3d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::numbers::mod_int::{BaseModInt, ModInt7};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    input.next_token();
    let a: usize = input.read();
    let b: usize = input.read();
    let c: usize = input.read();
    let k: usize = input.read();
    let p1: i32 = input.read();
    let p2: i32 = input.read();

    type Mod = ModInt7;
    let p1 = Mod::new(p1) / Mod::new(100);
    let p2 = Mod::new(p2) / Mod::new(100);
    let p3 = Mod::one() - p1 - p2;
    let rev2 = Mod::new(2).inv().unwrap();
    let mut ans = Arr3d::new(a + 1, b + 1, c + 1, None);
    let mut rec = RecursiveFunction3::new(|f, i, j, l| match ans[(i, j, l)] {
        None => {
            if i == 0 {
                ans[(i, j, l)] = Some(Mod::zero());
                Mod::zero()
            } else if j == 0 && l == 0 || i + j + l + k == a + b + c + 1 {
                ans[(i, j, l)] = Some(Mod::one());
                Mod::one()
            } else {
                let mut res = p1 * f.call(i - 1, j, l);
                if j != 0 {
                    res += p2 * f.call(i, j - 1, l);
                } else {
                    res += p2 * (f.call(i - 1, j, l) + f.call(i, j, l - 1)) * rev2;
                }
                if l != 0 {
                    res += p3 * f.call(i, j, l - 1);
                } else {
                    res += p3 * (f.call(i - 1, j, l) + f.call(i, j - 1, l)) * rev2;
                }
                ans[(i, j, l)] = Some(res);
                res
            }
        }
        Some(val) => val,
    });
    out_line!(rec.call(a, b, c));
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
