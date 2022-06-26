//{"name":"D - Jumping Takahashi 2","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_d","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n-10 0 1\n0 0 5\n10 0 1\n11 0 1\n","output":"2\n"},{"input":"7\n20 31 1\n13 4 3\n-10 -15 2\n34 26 5\n-2 39 4\n0 -50 1\n5 -20 2\n","output":"18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DJumpingTakahashi2"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let t = input.read_vec::<(i64, i64, i64)>(n);

    let dist = Arr2d::generate(n, n, |i, j| {
        ((t[i].0 - t[j].0).abs() + (t[i].1 - t[j].1).abs() + t[i].2 - 1) / t[i].2
    });
    let mut ans = None;
    for i in 0..n {
        let mut reached = BitSet::new(n);
        reached.set(i, true);
        let mut cur = 0;
        for _ in 1..n {
            let mut best = None;
            for j in 0..n {
                if !reached[j] {
                    continue;
                }
                for k in 0..n {
                    if !reached[k] {
                        best.minim((dist[(j, k)], k));
                    }
                }
            }
            let (len, to) = best.unwrap();
            cur.maxim(len);
            reached.set(to, true);
        }
        ans.minim(cur);
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
