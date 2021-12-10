//{"name":"A - Disjoint Set Union","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_a","interactive":false,"timeLimit":5000,"tests":[{"input":"4 7\n1 0 1\n0 0 1\n0 2 3\n1 0 1\n1 1 2\n0 0 2\n1 1 3\n","output":"0\n1\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADisjointSetUnion"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let q = input.read();

    let mut dsu = DSU::new(n);
    for _ in 0usize..q {
        let t: u32 = input.read();
        let u = input.read();
        let v = input.read();

        if t == 0 {
            dsu.join(u, v);
        } else {
            out_line!(if dsu.get(u) == dsu.get(v) { 1 } else { 0 });
        }
    }
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
