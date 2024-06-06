//{"name":"C - One Time Swap","group":"AtCoder - Monoxer Programming Contest 2024（AtCoder Beginner Contest 345）","url":"https://atcoder.jp/contests/abc345/tasks/abc345_c","interactive":false,"timeLimit":2000,"tests":[{"input":"abc\n","output":"3\n"},{"input":"aaaaa\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COneTimeSwap"}}}

use algo_lib::collections::slice_ext::qty::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    let qty = s
        .iter()
        .map(|c| (c - b'a') as usize)
        .collect::<Vec<_>>()
        .qty();
    let mut ans = 0;
    let mut have_big = false;
    for i in qty {
        if i > 1 {
            have_big = true;
        }
        ans += i * (s.len() - i);
    }
    ans /= 2;
    if have_big {
        ans += 1;
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
