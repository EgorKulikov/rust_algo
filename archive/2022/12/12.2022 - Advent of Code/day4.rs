//{"name":"day4","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day4"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut ans = 0;

    while !input.is_exhausted() {
        let s = input.read_string();
        let tokens = s.split(&['-', ',']).collect_vec();
        let a0 = tokens[0].parse::<i32>().unwrap();
        let a1 = tokens[1].parse::<i32>().unwrap();
        let b0 = tokens[2].parse::<i32>().unwrap();
        let b1 = tokens[3].parse::<i32>().unwrap();

        // if a0 <= b0 && b1 <= a1 || b0 <= a0 && a1 <= b1 {
        //     ans += 1;
        // }

        if !(a1 < b0 || b1 < a0) {
            ans += 1;
        }

        input.skip_whitespace();
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
