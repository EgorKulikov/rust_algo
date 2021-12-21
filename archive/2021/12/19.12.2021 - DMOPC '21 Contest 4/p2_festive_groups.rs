//{"name":"P2 - Festive Groups","group":"DMOJ - DMOPC '21 Contest 4","url":"https://dmoj.ca/problem/dmopc21c4p2","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n8 3 15 10 6 4 16\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P2FestiveGroups"}}}

use algo_lib::collections::vec_ext::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let j: Vec<usize> = input.read_vec(n);

    let mut q = j.as_slice().qty();
    let mut ans = 0;
    for i in 0..q.len() {
        if q[i] != 0 {
            ans += 1;
            for j in (i..q.len()).step_by(i) {
                q[j] = 0;
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
