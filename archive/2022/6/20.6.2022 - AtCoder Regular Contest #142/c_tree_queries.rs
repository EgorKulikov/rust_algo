//{"name":"C - Tree Queries","group":"AtCoder - AtCoder Regular Contest 142","url":"https://atcoder.jp/contests/arc142/tasks/arc142_c","interactive":true,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTreeQueries"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();

    let mut min_dist = None;
    let mut incident_to_1 = Vec::new();
    let mut incident_to_2 = Vec::new();
    for i in 3..=n {
        out_line!('?', 1, i);
        let d1 = input.read_usize();
        out_line!('?', 2, i);
        let d2 = input.read_usize();
        if d1 != 1 && d2 != 1 {
            continue;
        }
        if min_dist.minim(d1 + d2) {
            incident_to_1.clear();
            incident_to_2.clear();
        }
        if min_dist == Some(d1 + d2) {
            if d1 == 1 {
                incident_to_1.push(i);
            }
            if d2 == 1 {
                incident_to_2.push(i);
            }
        }
    }
    if incident_to_1.is_empty() || incident_to_2.is_empty() {
        out_line!('!', 1);
        return;
    }
    if min_dist != Some(3) {
        out_line!('!', min_dist);
        return;
    }
    out_line!('?', incident_to_1[0], incident_to_2[0]);
    if input.read_usize() == 1 {
        out_line!('!', 3);
    } else {
        out_line!('!', 1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    true
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
