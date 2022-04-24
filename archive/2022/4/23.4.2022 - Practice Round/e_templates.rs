//{"name":"E. Templates","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\nabc\nabd\nacd\n","output":"a??\n"},{"input":"3\nabc\nabc\nabc\n","output":"abc\n"},{"input":"4\n.123\n.456\n.789\n0123\n","output":"????\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETemplates"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Vec<Str> = input.read_vec(n);

    let len = s[0].len();
    for i in 0..len {
        let mut res = None;
        for s in &s {
            let c = s[i];
            match res {
                None => {
                    res = Some(Some(c));
                }
                Some(r) => {
                    if let Some(d) = r {
                        if c != d {
                            res = Some(None);
                        }
                    }
                }
            }
        }
        out!(match res.unwrap() {
            None => '?',
            Some(c) => c as char,
        });
    }
    out_line!();
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
