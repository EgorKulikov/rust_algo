//{"name":"A. Новый палиндром","group":"Codeforces - Educational Codeforces Round 148 (Rated for Div. 2)","url":"https://codeforces.com/contest/1832/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\ncodedoc\ngg\naabaa\n","output":"YES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANoviiPalindrom"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::StrReader;

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read_str();

    for c in s.iter().take(s.len() / 2) {
        if c != s[0] {
            out_line!(true);
            return;
        }
    }
    out_line!(false);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
