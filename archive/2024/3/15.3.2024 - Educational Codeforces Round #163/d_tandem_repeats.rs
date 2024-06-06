//{"name":"D. Tandem Repeats?","group":"Codeforces - Educational Codeforces Round 163 (Rated for Div. 2)","url":"https://codeforces.com/contest/1948/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\nzaabaabz\n?????\ncode?????s\ncodeforces\n","output":"6\n4\n10\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTandemRepeats"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    for i in (1..=s.len() / 2).rev() {
        let mut cur = 0;
        for j in i..s.len() {
            if s[j] != b'?' && s[j - i] != b'?' && s[j] != s[j - i] {
                cur = 0;
            } else {
                cur += 1;
                if cur == i {
                    out.print_line(i * 2);
                    return;
                }
            }
        }
    }
    out.print_line(0);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
