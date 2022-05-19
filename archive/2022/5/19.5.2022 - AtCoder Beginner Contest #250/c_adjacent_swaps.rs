//{"name":"C - Adjacent Swaps","group":"AtCoder - AtCoder Beginner Contest 250","url":"https://atcoder.jp/contests/abc250/tasks/abc250_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1\n2\n3\n4\n5\n","output":"1 2 3 5 4\n"},{"input":"7 7\n7\n7\n7\n7\n7\n7\n7\n","output":"1 2 3 4 5 7 6\n"},{"input":"10 6\n1\n5\n2\n9\n6\n6\n","output":"1 2 3 4 5 7 6 8 10 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAdjacentSwaps"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let q = input.read_usize();

    let mut pos = (0..n).collect_vec();
    let mut ans = pos.clone();
    for _ in 0..q {
        let x = input.read_usize() - 1;
        if pos[x] == n - 1 {
            ans.swap(pos[x], pos[x] - 1);
        } else {
            ans.swap(pos[x], pos[x] + 1);
        }
        let other = ans[pos[x]];
        pos.swap(x, other);
    }
    out_line!(ans.inc_by_one());
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
