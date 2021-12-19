//{"name":"C - Graph Isomorphism","group":"AtCoder - M-SOLUTIONS Programming Contest 2021(AtCoder Beginner Contest 232)","url":"https://atcoder.jp/contests/abc232/tasks/abc232_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4\n1 2\n1 3\n1 4\n3 4\n1 3\n1 4\n2 3\n3 4\n","output":"Yes\n"},{"input":"5 6\n1 2\n1 3\n1 4\n3 4\n3 5\n4 5\n1 2\n1 3\n1 4\n1 5\n3 5\n4 5\n","output":"No\n"},{"input":"8 0\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGraphIsomorphism"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashSet;

fn solve(input: &mut Input) {
    let n = input.read();
    let m = input.read();
    let edges1 = input.read_vec::<(usize, usize)>(m).dec_by_one();
    let edges2 = input.read_vec::<(usize, usize)>(m).dec_by_one();

    let set = edges1.into_iter().collect::<HashSet<_>>();
    let mut p = Permutation::new_ident(n);
    loop {
        let mut good = true;
        for (u, v) in edges2.iter().cloned() {
            if !set.contains(&(p[u], p[v])) && !set.contains(&(p[v], p[u])) {
                good = false;
                break;
            }
        }
        if good {
            out_line!("Yes");
            return;
        }
        match p.next() {
            None => break,
            Some(per) => p = per,
        }
    }
    out_line!("No");
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
