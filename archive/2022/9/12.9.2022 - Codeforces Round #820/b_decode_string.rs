//{"name":"B. Decode String","group":"Codeforces - Codeforces Round #820 (Div. 3)","url":"https://codeforces.com/contest/1729/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n6\n315045\n4\n1100\n7\n1213121\n6\n120120\n18\n315045615018035190\n7\n1111110\n7\n1111100\n5\n11111\n4\n2606\n","output":"code\naj\nabacaba\nll\ncodeforces\naaaak\naaaaj\naaaaa\nzf\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDecodeString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    input.read_usize();
    let t: Str = input.read();

    let mut s = Vec::new();
    let mut double = false;
    let mut single = false;
    let mut val = 0;
    for c in t.iter().rev() {
        if double {
            double = false;
            val = c - b'0';
            single = true;
        } else if single {
            s.push(b'a' + (val + 10 * (c - b'0')) - 1);
            single = false;
        } else if c == b'0' {
            double = true;
        } else {
            s.push(b'a' + c - b'1');
        }
    }
    s.reverse();
    out_line!(Str::from(s));
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
