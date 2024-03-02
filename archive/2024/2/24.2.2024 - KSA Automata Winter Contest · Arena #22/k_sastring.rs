//{"name":"KSA String","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/contest/problem/1238/2","interactive":false,"timeLimit":1000,"tests":[{"input":"KKSKASKA\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KSAString"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_str();

    let mut ans = None;
    let order = Str::from("KSA");
    for i in 0..3 {
        let mut next = i;
        let mut len = i;
        let mut cur = i;
        for c in x.iter() {
            if c == order[next] {
                next += 1;
                len += 1;
                if next == 3 {
                    next = 0;
                }
            } else {
                cur += 1;
            }
        }
        if len > x.len() {
            cur += len - x.len();
            len = x.len();
        }
        cur += x.len() - len;
        ans.minim(cur);
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
