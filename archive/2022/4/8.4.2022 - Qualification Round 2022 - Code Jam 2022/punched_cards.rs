//{"name":"Punched Cards","group":"Google Coding Competitions - Qualification Round 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a4621b","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n3 4\n2 2\n2 3\n","output":"Case #1:\n..+-+-+-+\n..|.|.|.|\n+-+-+-+-+\n|.|.|.|.|\n+-+-+-+-+\n|.|.|.|.|\n+-+-+-+-+\nCase #2:\n..+-+\n..|.|\n+-+-+\n|.|.|\n+-+-+\nCase #3:\n..+-+-+\n..|.|.|\n+-+-+-+\n|.|.|.|\n+-+-+-+\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PunchedCards"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let r = input.read_usize();
    let c = input.read_usize();

    let mut ans = Arr2d::new(2 * r + 1, 2 * c + 1, '.');
    for i in 0..=r {
        for j in 0..=c {
            if i > 0 || j > 0 {
                ans[(2 * i, 2 * j)] = '+';
            }
        }
    }
    for i in 0..=r {
        for j in 0..c {
            if i > 0 || j > 0 {
                ans[(2 * i, 2 * j + 1)] = '-';
            }
        }
    }
    for i in 0..r {
        for j in 0..=c {
            if i > 0 || j > 0 {
                ans[(2 * i + 1, 2 * j)] = '|';
            }
        }
    }
    out_line!(format!("Case #{}: ", test_case));
    output().print_table(&ans);
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
