//{"name":"C. Team Building","group":"Codeforces - Constructor Open Cup 2023","url":"https://constructor2023.contest.codeforces.com/group/sVRDLercWX/contest/431163/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 1 2\n123\n5 3 5\n31231\n4 1 3\n2122\n6 2 4\n332121\n3 2 2\n123\n2 1 2\n22\n","output":"YES\nNO\nNO\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTeamBuilding"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let l = input.read_size();
    let r = input.read_size();
    let c = input.read_vec::<char>(n);

    let cap = c.iter().count_eq(&&'1');
    let flex = c.iter().count_eq(&&'3');
    let min_teams = (n + r - 1) / r;
    let max_teams = n / l;
    out_line!(cap + flex >= min_teams && cap <= max_teams && min_teams <= max_teams);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
