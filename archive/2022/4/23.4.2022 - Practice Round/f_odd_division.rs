//{"name":"F. Odd Division","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"1317819\n","output":"6\n"},{"input":"040\n","output":"-1\n"},{"input":"101\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FOddDivision"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    if s[0] == b'0' || (s[s.len() - 1] - b'0') % 2 == 0 {
        out_line!(-1);
        return;
    }
    let mut ans = 1;
    for (&c, &d) in s.as_slice().consecutive_iter() {
        let c = c - b'0';
        let d = d - b'0';

        if c % 2 == 1 && d != 0 {
            ans += 1;
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
