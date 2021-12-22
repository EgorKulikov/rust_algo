//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/algorithm/card-sum-6710882a/","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n5 3\n-2 1 0 2 2\n","output":"1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let x: i64 = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let sum: i64 = a.into_iter().sum();
    out_line!((sum.abs() + x - 1) / x);
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
