//{"name":"D - Bank","group":"AtCoder - AtCoder Beginner Contest 294","url":"https://atcoder.jp/contests/abc294/tasks/abc294_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 10\n1\n1\n3\n2 1\n1\n2 3\n3\n1\n2 2\n3\n","output":"1\n2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBank"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::BTreeSet;

fn solve(input: &mut Input) {
    let _n = input.read_size();
    let q = input.read_size();

    let mut next = 1;
    let mut set = BTreeSet::new();
    for _ in 0..q {
        let t = input.read_size();
        match t {
            1 => {
                set.insert(next);
                next += 1;
            }
            2 => {
                let id = input.read_size();
                set.remove(&id);
            }
            3 => {
                out_line!(set.iter().next().unwrap());
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
