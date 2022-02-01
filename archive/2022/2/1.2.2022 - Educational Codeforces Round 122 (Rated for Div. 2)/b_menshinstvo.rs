//{"name":"B. Меньшинство","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n01\n1010101010111\n00110001000\n1\n","output":"0\n5\n3\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMenshinstvo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let num_zero = s.iter().filter(|&c| c == b'0').count();
    let num_one = s.iter().filter(|&c| c == b'1').count();
    if num_zero == num_one {
        out_line!(num_zero - 1);
    } else {
        out_line!(num_zero.min(num_one));
    }
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
