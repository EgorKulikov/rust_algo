//{"name":"F - Variety of Digits","group":"AtCoder - HHKB Programming Contest 2022（AtCoder Beginner Contest 235）","url":"https://atcoder.jp/contests/abc235/tasks/abc235_f","interactive":false,"timeLimit":2000,"tests":[{"input":"104\n2\n0 1\n","output":"520\n"},{"input":"999\n4\n1 2 3 4\n","output":"0\n"},{"input":"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\n5\n0 2 4 6 8\n","output":"397365274\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FVarietyOfDigits"}}}

use algo_lib::collections::legacy_fill::LegacyFill;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::mem::swap;

fn solve(input: &mut Input) {
    let n: Str = input.read();
    let m = input.read_usize();
    let c = input.read_usize_vec(m);

    type Mod = ModIntF;
    let mut mask_top = 0;
    let mut sum = Mod::zero();
    let mut qty = vec![(Mod::zero(), Mod::zero()); 1 << 10];
    let mut next = vec![(Mod::zero(), Mod::zero()); 1 << 10];
    for c in n {
        next.legacy_fill((Mod::zero(), Mod::zero()));
        let c = (c - b'0').into_usize();
        for j in if mask_top == 0 { 1 } else { 0 }..c {
            next[mask_top.with_bit(j)].0 += Mod::one();
            next[mask_top.with_bit(j)].1 += sum * Mod::new(10) + Mod::from_index(j);
        }
        if mask_top != 0 {
            for j in 1..10 {
                next[1 << j].0 += Mod::one();
                next[1 << j].1 += Mod::from_index(j);
            }
        }
        mask_top.set_bit(c);
        sum *= Mod::new(10);
        sum += Mod::from_index(c);
        for i in 0..qty.len() {
            for j in 0..10 {
                next[i.with_bit(j)].0 += qty[i].0;
                next[i.with_bit(j)].1 += qty[i].1 * Mod::new(10) + qty[i].0 * Mod::from_index(j);
            }
        }
        swap(&mut qty, &mut next);
    }
    qty[mask_top].0 += Mod::one();
    qty[mask_top].1 += sum;
    let mut req = 0;
    for i in c {
        req.set_bit(i);
    }
    let mut ans = Mod::zero();
    for (i, (_, v)) in qty.into_iter().enumerate() {
        if (i & req) == req {
            ans += v;
        }
    }
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
