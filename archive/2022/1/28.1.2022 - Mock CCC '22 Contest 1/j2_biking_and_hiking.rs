//{"name":"J2 - Biking and Hiking","group":"DMOJ - Mock CCC '22 Contest 1","url":"https://dmoj.ca/problem/mccc3j2","interactive":false,"timeLimit":250,"tests":[{"input":"11 1\nFUDDDUDUUUF\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"J2BikingAndHiking"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.read_usize();
    let mut k = input.read_usize();
    let s: Str = input.read();

    let mut ans = 0;
    for c in s {
        if c == b'D' {
            k += 1;
        }
        if c == b'U' && k != 0 {
            k -= 1;
        }
        if k == 0 {
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
