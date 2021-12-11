//{"name":"D - Neighbors","group":"AtCoder - Panasonic Programming Contest 2021(AtCoder Beginner Contest 231)","url":"https://atcoder.jp/contests/abc231/tasks/abc231_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 3\n2 3\n","output":"Yes\n"},{"input":"4 3\n1 4\n2 4\n3 4\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNeighbors"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let m = input.read();
    let p: Vec<(usize, usize)> = input.read_vec(m);

    let mut dsu = DSU::new(n);
    let mut deg = vec![0usize; n];
    for (a, b) in p {
        if !dsu.join(a - 1, b - 1) {
            out_line!("No");
            return;
        }
        deg[a - 1] += 1;
        deg[b - 1] += 1;
    }
    if deg.into_iter().max().unwrap() > 2 {
        out_line!("No");
    } else {
        out_line!("Yes");
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
