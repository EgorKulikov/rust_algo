//{"name":"A. Mobile Internet Package","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"100\n140\n2\n120\n","output":"180\n"},{"input":"30\n150\n1\n10\n","output":"150\n"},{"input":"75\n150\n150\n150\n","output":"11400\n"},{"input":"70\n150\n125\n70\n","output":"150\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMobileInternetPackage"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read_int();
    let c1 = input.read_int();
    let c2 = input.read_int();
    let p = input.read_int();

    let ans = c1 + c2 * 0.max(p - s);
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
