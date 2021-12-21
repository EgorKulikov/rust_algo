//{"name":"P1 - Calendar Management","group":"DMOJ - SAC '22 Code Challenge 2","url":"https://dmoj.ca/problem/sac22cc2p1","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5\n3 Physics\n6 English\n5 Rock_throwing\n1\n3\n6\n7\n8\n","output":"Physics\nEnglish\nRock_throwing\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P1CalendarManagement"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::ops::Deref;

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read();
    let k = input.read();
    let mut assessments: Vec<(usize, String)> = input.read_vec(a);
    let c: Vec<usize> = input.read_vec(k);

    assessments
        .sort_by(|(d1, _), (d2, _)| c.deref().lower_bound(d1).cmp(&c.deref().lower_bound(d2)));
    while let Some((day, _)) = assessments.last() {
        if day <= c.last().unwrap() {
            break;
        } else {
            assessments.pop();
        }
    }
    output().print_per_line(&assessments.into_iter().map(|(_, s)| s).collect_vec());
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
