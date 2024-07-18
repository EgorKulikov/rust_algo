//{"name":"B - Frequency","group":"AtCoder - AtCoder Beginner Contest 338","url":"https://atcoder.jp/contests/abc338/tasks/abc338_b","interactive":false,"timeLimit":2000,"tests":[{"input":"frequency\n","output":"e\n"},{"input":"atcoder\n","output":"a\n"},{"input":"pseudopseudohypoparathyroidism\n","output":"o\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFrequency"}}}

use algo_lib::collections::iter_ext::min_max::IterMinMaxPos;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    let mut qty = vec![0; 26];
    for c in s {
        qty[c as usize - 'a' as usize] += 1;
    }
    let ans = qty.into_iter().max_position().unwrap();
    out.print_line((ans + ('a' as usize)) as u8 as char);
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
                solve(&mut input, &mut output, i, &pre_calc);
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
