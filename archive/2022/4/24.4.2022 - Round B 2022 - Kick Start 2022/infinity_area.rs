//{"name":"Infinity Area","group":"Google Coding Competitions - Round B 2022 - Kick Start 2022","url":"https://codingcompetitions.withgoogle.com/kickstart/round/00000000008caa74/0000000000acf079","interactive":false,"timeLimit":20000,"tests":[{"input":"2\n1 3 6\n5 2 5\n","output":"Case #1: 31.415927\nCase #2: 455.530935\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"InfinityArea"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::f64::consts::PI;

use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let mut r = input.read_long();
    let a = input.read_long();
    let b = input.read_long();

    let mut ans = 0;
    while r != 0 {
        ans += r * r;
        r *= a;
        ans += r * r;
        r /= b;
    }
    out_line!(format!("Case #{}: ", test_case), (ans as f64) * PI);
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
