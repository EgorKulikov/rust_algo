//{"name":"Pancake Deque","group":"Google Coding Competitions - Round 1B 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/000000000087711b/0000000000acd59d","interactive":false,"timeLimit":20000,"tests":[{"input":"4\n2\n1 5\n4\n1 4 2 3\n5\n10 10 10 10 10\n4\n7 1 3 1000000\n","output":"Case #1: 2\nCase #2: 3\nCase #3: 5\nCase #4: 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PancakeDeque"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::collections::VecDeque;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let d = input.read_int_vec(n);

    let mut level = 0;
    let mut ans = 0;
    let mut q = d.into_iter().collect::<VecDeque<_>>();
    while !q.is_empty() {
        let front = *q.front().unwrap();
        let back = *q.back().unwrap();
        let cur = if front < back {
            q.pop_front();
            front
        } else {
            q.pop_back();
            back
        };
        if cur >= level {
            ans += 1;
            level = cur;
        }
    }
    out_line!(format!("Case #{}:", test_case), ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
    // input.skip_whitespace();
    // !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
