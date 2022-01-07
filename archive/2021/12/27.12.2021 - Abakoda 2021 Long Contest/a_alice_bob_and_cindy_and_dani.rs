//{"name":"A. Alice, Bob, and Cindy, and Dani","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"Alice Bob Cindy\n","output":"Dani\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAliceBobAndCindyAndDani"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Vec<String> = input.read_vec(3);

    let mut ans = ["Alice", "Bob", "Cindy", "Dani"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
    for c in s {
        ans.remove(&c);
    }
    out_line!(ans.into_iter().next().unwrap());
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
