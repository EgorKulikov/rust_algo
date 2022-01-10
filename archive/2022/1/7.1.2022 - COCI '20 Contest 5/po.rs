//{"name":"#2 - Po","group":"DMOJ - COCI '20 Contest 5","url":"https://dmoj.ca/problem/coci20c5p2","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 2 2\n","output":"1\n"},{"input":"5\n2 3 3 3 2\n","output":"2\n"},{"input":"6\n1 2 3 2 1 3\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Po"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let mut stack = VecDeque::new();
    stack.push_back(0);
    let mut ans = 0;
    for i in a {
        while stack.back().unwrap() > &i {
            stack.pop_back();
        }
        if stack.back().unwrap() != &i {
            ans += 1;
            stack.push_back(i);
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
