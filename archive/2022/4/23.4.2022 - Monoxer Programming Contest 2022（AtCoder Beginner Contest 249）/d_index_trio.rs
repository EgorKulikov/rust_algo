//{"name":"D - Index Trio","group":"AtCoder - Monoxer Programming Contest 2022（AtCoder Beginner Contest 249）","url":"https://atcoder.jp/contests/abc249/tasks/abc249_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6 2 3\n","output":"2\n"},{"input":"1\n2\n","output":"0\n"},{"input":"10\n1 3 2 4 6 8 2 2 3 7\n","output":"62\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DIndexTrio"}}}

use algo_lib::collections::vec_ext::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    let q = a.qty();
    let mut ans = 0;
    for i in 1..q.len() {
        if i * i >= q.len() {
            break;
        }
        if q[i] != 0 {
            ans += q[i] * q[i] * q[i * i];
            for j in i + 1..q.len() {
                if i * j >= q.len() {
                    break;
                }
                ans += 2 * q[i] * q[j] * q[i * j];
            }
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
