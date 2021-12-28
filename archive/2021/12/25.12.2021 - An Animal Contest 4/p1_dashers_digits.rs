//{"name":"P1 - Dasher's Digits","group":"DMOJ - An Animal Contest 4","url":"https://dmoj.ca/problem/aac4p1","interactive":false,"timeLimit":2000,"tests":[{"input":"7 1\nUS0AMOG\n2\n","output":"AMOGUS\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P1DashersDigits"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    input.next_token();
    let m = input.read();
    let x: Str = input.read();
    let a: Vec<u32> = input.read_vec(m);

    let max_a = *a.iter().max().unwrap();
    let pos = m - a.into_iter().rev().find(max_a).unwrap();
    let mut zeros = 0;
    for (i, c) in x.iter().enumerate() {
        if c == b'0' {
            zeros += 1;
            if zeros == pos {
                out_line!(x[i..]
                    .iter()
                    .chain(x[..i].iter())
                    .filter(|c| **c != b'0')
                    .collect::<Str>());
                return;
            }
        }
    }
    unreachable!();
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
