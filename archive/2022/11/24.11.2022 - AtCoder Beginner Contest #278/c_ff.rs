//{"name":"C - FF","group":"AtCoder - AtCoder Beginner Contest 278","url":"https://atcoder.jp/contests/abc278/tasks/abc278_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 9\n1 1 2\n3 1 2\n1 2 1\n3 1 2\n1 2 3\n1 3 2\n3 1 3\n2 1 2\n3 1 2\n","output":"No\nYes\nNo\nNo\n"},{"input":"2 8\n1 1 2\n1 2 1\n3 1 2\n1 1 2\n1 1 2\n1 1 2\n2 1 2\n3 1 2\n","output":"Yes\nNo\n"},{"input":"10 30\n3 1 6\n3 5 4\n1 6 1\n3 1 7\n3 8 4\n1 1 6\n2 4 3\n1 6 5\n1 5 6\n1 1 8\n1 8 1\n2 3 10\n1 7 6\n3 5 6\n1 6 7\n3 6 7\n1 9 5\n3 8 6\n3 3 8\n2 6 9\n1 7 1\n3 10 8\n2 9 2\n1 10 9\n2 6 10\n2 6 8\n3 1 6\n3 1 8\n2 8 5\n1 9 10\n","output":"No\nNo\nNo\nNo\nYes\nYes\nNo\nNo\nNo\nYes\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFF"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input) {
    input.read_usize();
    let q = input.read_usize();

    let mut follows = HashSet::new();
    set_bool_output(BoolOutput::YesNo);
    for _ in 0..q {
        let t = input.read_usize();
        let a = input.read_usize();
        let b = input.read_usize();

        match t {
            1 => {
                follows.insert((a, b));
            }
            2 => {
                follows.remove(&(a, b));
            }
            3 => {
                out_line!(follows.contains(&(a, b)) && follows.contains(&(b, a)));
            }
            _ => unreachable!(),
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
