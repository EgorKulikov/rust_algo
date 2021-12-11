//{"name":"B - Election","group":"AtCoder - Panasonic Programming Contest 2021(AtCoder Beginner Contest 231)","url":"https://atcoder.jp/contests/abc231/tasks/abc231_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nsnuke\nsnuke\ntakahashi\ntakahashi\ntakahashi\n","output":"takahashi\n"},{"input":"5\ntakahashi\ntakahashi\naoki\ntakahashi\nsnuke\n","output":"takahashi\n"},{"input":"1\na\n","output":"a\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BElection"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashMap;

fn solve(input: &mut Input) {
    let n = input.read();
    let s: Vec<String> = input.read_vec(n);

    let mut m = HashMap::new();
    for c in s {
        if !m.contains_key(&c) {
            m.insert(c.clone(), 0);
        }
        *m.get_mut(&c).unwrap() += 1;
    }
    out_line!(m.into_iter().map(|(c, i)| (i, c)).max().unwrap().1);
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
