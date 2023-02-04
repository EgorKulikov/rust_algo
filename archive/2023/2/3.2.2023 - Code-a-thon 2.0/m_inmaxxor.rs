//{"name":"MIN MAX XOR","group":"CodeChef - CDAT2023","url":"https://www.codechef.com/CDAT2023/problems/MINMAXOR_","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n11111\n11101\n00000\n100\n100\n100\n1\n1\n0\n","output":"10000\n100\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MINMAXXOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read::<Str>();
    let n = s.len();
    let a = s.iter().filter(|&c| c == b'1').count();
    let b = input.read::<Str>().iter().filter(|&c| c == b'1').count();
    let c = input.read::<Str>().iter().filter(|&c| c == b'1').count();

    let d = if a > b { a - b } else { b - a };
    let ans = if c + d < n { c + d } else { 2 * n - (c + d) };
    for _ in 0..ans {
        out!('1');
    }
    for _ in ans..n {
        out!('0');
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
