//{"name":"E - Rook Path","group":"AtCoder - M-SOLUTIONS Programming Contest 2021(AtCoder Beginner Contest 232)","url":"https://atcoder.jp/contests/abc232/tasks/abc232_e","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2 2\n1 2 2 1\n","output":"2\n"},{"input":"1000000000 1000000000 1000000\n1000000000 1000000000 1000000000 1000000000\n","output":"24922282\n"},{"input":"3 3 3\n1 3 3 3\n","output":"9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERookPath"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModIntF};
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};
use std::mem::swap;

fn solve(input: &mut Input) {
    let h: i32 = input.read();
    let w: i32 = input.read();
    let k = input.read();
    let x1: i32 = input.read();
    let y1: i32 = input.read();
    let x2: i32 = input.read();
    let y2: i32 = input.read();

    type Mod = ModIntF;
    let mut state = Arr2d::new(2, 2, Mod::zero());
    state[(if x1 == x2 { 0 } else { 1 }, if y1 == y2 { 0 } else { 1 })] = Mod::one();
    let mut next = Arr2d::new(2, 2, Mod::zero());
    let q_x = [Mod::one(), Mod::new(h - 1)];
    let q_y = [Mod::one(), Mod::new(w - 1)];
    for _ in 0..k {
        next.fill(Mod::zero());
        for i in 0..2 {
            for j in 0..2 {
                next[(i, j)] += (q_x[i] - Mod::one() + q_y[j] - Mod::one()) * state[(i, j)];
                next[(1 - i, j)] += q_x[1 - i] * state[(i, j)];
                next[(i, 1 - j)] += q_y[1 - j] * state[(i, j)];
            }
        }
        swap(&mut state, &mut next);
    }
    out_line!(state[(0, 0)]);
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
