//{"name":"day25","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day25"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut ans = 0;

    while !input.is_exhausted() {
        let s: Str = input.read();
        let mut cur = 1i64;
        for c in s.iter().rev() {
            let d = match c {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                _ => unreachable!(),
            };
            ans += cur * d;
            cur *= 5;
        }
        input.skip_whitespace();
    }

    let mut cur = 1;
    let mut delta = 0;
    let mut max_delta = Vec::new();
    let mut v = Vec::new();
    while delta < ans {
        max_delta.push(delta);
        v.push(cur);
        delta += 2 * cur;
        cur *= 5;
    }
    for (next, cur) in max_delta.into_iter().zip(v.into_iter()).rev() {
        for i in -2..=2 {
            if (ans - i * cur).abs() <= next {
                ans -= i * cur;
                out!(match i {
                    -2 => '=',
                    -1 => '-',
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    _ => unreachable!(),
                });
                break;
            }
        }
    }
    out_line!();
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
