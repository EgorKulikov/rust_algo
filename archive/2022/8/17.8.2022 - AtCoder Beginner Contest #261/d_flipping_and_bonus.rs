//{"name":"D - Flipping and Bonus","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_d","interactive":false,"timeLimit":2000,"tests":[{"input":"6 3\n2 7 1 8 2 8\n2 10\n3 1\n5 5\n","output":"48\n"},{"input":"3 2\n1000000000 1000000000 1000000000\n1 1000000000\n3 1000000000\n","output":"5000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DFlippingAndBonus"}}}

use algo_lib::collections::legacy_fill::LegacyFill;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let x = input.read_long_vec(n);
    let bonus = input.read_vec::<(usize, i64)>(m);

    let mut add = vec![0; n + 1];
    for (c, y) in bonus {
        add[c] += y;
    }
    const INF: i64 = std::i64::MIN;
    let mut cur = vec![INF; n + 1];
    cur[0] = 0;
    let mut next = vec![INF; n + 1];
    for i in x {
        next.legacy_fill(INF);
        for j in 0..n {
            next[0].maxim(cur[j]);
            next[j + 1].maxim(cur[j] + i + add[j + 1]);
        }
        swap(&mut cur, &mut next);
    }
    out_line!(cur.into_iter().max());
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
