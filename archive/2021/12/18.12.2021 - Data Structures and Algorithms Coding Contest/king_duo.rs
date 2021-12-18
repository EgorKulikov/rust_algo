//{"name":"King Duo","group":"HackerEarth - Data Structures and Algorithms Coding Contest","url":"https://www.hackerearth.com/challenges/competitive/data-structures-and-algorithms-coding-contest-dec/algorithm/king-duo-9ac93811/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2\n3\n","output":"0\n32\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KingDuo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: i64 = input.read();

    let ans = 4 * (n * n - 4).max(0)
        + 4 * (n - 2) * (n * n - 6).max(0)
        + (n - 2) * (n - 2) * (n * n - 9).max(0);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
