//{"name":"B. Cinema","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"1 20\n10 10\n","output":"YES\n"},{"input":"11 2\n7 3\n","output":"YES\n"},{"input":"10 18\n5 15\n","output":"YES\n"},{"input":"42 13\n13 37\n","output":"YES\n"},{"input":"10 10\n1 15\n","output":"NO\n"},{"input":"70 80\n75 75\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCinema"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read_int();
    let b = input.read_int();
    let c = input.read_int();
    let d = input.read_int();

    out_line!(a >= c + d || b >= c + d || a >= c && b >= d || a >= d && b >= c);
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
