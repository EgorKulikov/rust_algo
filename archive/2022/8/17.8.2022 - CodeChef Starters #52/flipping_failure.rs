//{"name":"Flipping Failure","group":"CodeChef - START52A","url":"https://www.codechef.com/START52A/problems-old/FLIPFAIL","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n1010\n101\n100100\n","output":"1\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FlippingFailure"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let k = s.iter().count_eq(&b'0');
    let mut cur = 0;
    for c in s.iter().take(k) {
        if c == b'1' {
            cur += 1;
        }
    }
    let mut ans = cur;
    for (i, c) in s.iter().enumerate().skip(k) {
        if c == b'1' {
            cur += 1;
        }
        if s[i - k] == b'1' {
            cur -= 1;
        }
        ans = ans.min(cur);
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
