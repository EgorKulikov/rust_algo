//{"name":"ASeDatAb","group":"Google Coding Competitions - Round 1B 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/000000000087711b/0000000000acd29b","interactive":true,"timeLimit":10000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASeDatAb"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::process::exit;

use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut last = 1;
    loop {
        for _ in 0..last {
            out!(1);
        }
        for _ in last..8 {
            out!(0);
        }
        out_line!();
        last = input.read_int();
        if last == -1 {
            exit(0);
        }
        if last == 0 {
            return;
        }
    }
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
