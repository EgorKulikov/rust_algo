//{"name":"B. Buzz Progression","group":"Codeforces - Practice Round","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378187/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4 10\n","output":"64\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBuzzProgression"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut x = input.read_int() - 1;
    let n = input.read_int();

    let mut res = 0;
    for i in 1..=n {
        if i % 5 == 0 {
            x -= 2;
        } else {
            x += 1;
        }
        res += x;
    }
    out_line!(res);
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
