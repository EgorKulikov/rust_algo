//{"name":"D. Возвращение робота-пылесоса","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"https://codeforces.com/contest/1623/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 2 1 1 2 1 25\n3 3 1 2 2 2 25\n10 10 1 1 10 10 75\n10 10 10 10 1 1 75\n5 5 1 3 2 2 10\n97 98 3 5 41 43 50\n","output":"3\n3\n15\n15\n332103349\n99224487\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVozvrashchenieRobotaPilesosa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt7};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let rb = input.read_usize();
    let cb = input.read_usize();
    let rd = input.read_usize();
    let cd = input.read_usize();
    let p = input.read_int();

    let mut cleans = Vec::with_capacity(4 * n + 4 * m - 8);
    for (rb, rd, n, m) in [(rb, rd, n, m), (cb, cd, m, n)] {
        if rd >= rb {
            for i in 0..2 * m - 2 {
                cleans.push(rd - rb + i * (2 * n - 2));
            }
        } else {
            for i in 0..2 * m - 2 {
                cleans.push(2 * n - 2 + rd - rb + i * (2 * n - 2));
            }
        }
        for i in 0..2 * m - 2 {
            cleans.push(2 * n - rd - rb + i * (2 * n - 2));
        }
    }
    cleans.sort_unstable();
    cleans.dedup();
    let length = (2 * n - 2) * (2 * m - 2);
    if cleans.last().cloned().unwrap() == length {
        cleans.pop();
    }

    type Mod = ModInt7;
    let p = Mod::new(p) / Mod::new(100);
    let mut ans = Mod::zero();
    let mut rem = Mod::one();
    for i in cleans {
        ans += Mod::new_from_long(i.into_i64()) * p * rem;
        rem *= Mod::one() - p;
    }
    ans = (ans + rem * Mod::new_from_long(length.into_i64())) / (Mod::one() - rem);
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
