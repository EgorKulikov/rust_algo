//{"name":"A. Div. 7","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n42\n23\n377\n","output":"42\n28\n777\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADiv7"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut n = input.read_unsigned();

    if n % 7 == 0 {
        out_line!(n);
    } else {
        n -= n % 10;
        while n % 7 != 0 {
            n += 1;
        }
        out_line!(n);
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
