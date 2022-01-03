//{"name":"D. Зимний поход","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n0 8\n1 99\n2\n0 0 0 0\n0 0 0 0\n9 9 2 2\n9 9 9 9\n2\n0 0 4 2\n0 0 2 4\n4 2 4 2\n2 4 2 4\n4\n0 0 0 0 0 0 0 2\n0 0 0 0 0 0 2 0\n0 0 0 0 0 2 0 0\n0 0 0 0 2 0 0 0\n0 0 0 2 2 0 2 2\n0 0 2 0 1 6 2 1\n0 2 0 0 2 4 7 4\n2 0 0 0 2 0 1 6\n","output":"100\n22\n14\n42\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DZimniiPokhod"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let c: Arr2d<u64> = input.read_table(2 * n, 2 * n);

    let mut ans = c[(0, n)];
    ans.minim(c[(0, 2 * n - 1)]);
    ans.minim(c[(n - 1, n)]);
    ans.minim(c[(n - 1, 2 * n - 1)]);
    ans.minim(c[(n, 0)]);
    ans.minim(c[(n, n - 1)]);
    ans.minim(c[(2 * n - 1, 0)]);
    ans.minim(c[(2 * n - 1, n - 1)]);
    for i in n..2 * n {
        for j in n..2 * n {
            ans += c[(i, j)];
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
