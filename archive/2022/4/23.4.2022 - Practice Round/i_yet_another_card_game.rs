//{"name":"I. Yet Another Card Game","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/I","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n2 3\n4 1\n","output":"4\n"},{"input":"3\n1 2 5\n3 6 4\n","output":"11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IYetAnotherCardGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);
    let b = input.read_usize_vec(n);

    let mut a = a.into_iter().collect::<VecDeque<_>>();
    let mut b = b.into_iter().collect::<VecDeque<_>>();
    for i in 0..1000000 {
        match a.pop_front() {
            None => {
                out_line!(i);
                return;
            }
            Some(c) => match b.pop_front() {
                None => {
                    out_line!(i);
                    return;
                }
                Some(d) => {
                    if c > d {
                        a.push_back(c);
                        a.push_back(d);
                    } else {
                        b.push_back(d);
                        b.push_back(c);
                    }
                }
            },
        }
    }
    out_line!(-1);
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
