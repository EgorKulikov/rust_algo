//{"name":"B. И не ноль","group":"Codeforces - Codeforces Global Round 18","url":"https://codeforces.com/contest/1615/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 2\n2 8\n4 5\n1 5\n100000 200000\n","output":"1\n3\n0\n2\n31072\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BINeNol"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let l: i32 = input.read();
    let r: i32 = input.read();

    let mut ans = r - l + 1;
    for i in 0..20 {
        let qty = |mut u: i32| -> i32 {
            u -= 1 << i;
            if u < 0 {
                return 0;
            }
            let full = u / (1 << (i + 1));
            u %= 1 << (i + 1);
            (full << i) + (u + 1).min(1 << i)
        };
        let q = qty(r) - qty(l - 1);
        ans.minim(r - l + 1 - q);
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
