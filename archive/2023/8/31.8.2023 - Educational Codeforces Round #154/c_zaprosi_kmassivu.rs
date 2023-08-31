//{"name":"C. Запросы к массиву","group":"Codeforces - Educational Codeforces Round 154 (Rated for Div. 2)","url":"https://codeforces.com/contest/1861/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n++1\n+++1--0\n+0\n0\n++0-+1-+0\n++0+-1+-0\n+1-+0\n","output":"YES\nNO\nNO\nNO\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CZaprosiKMassivu"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    let mut sorted_prefix = 0;
    let mut non_sorted_at = None;
    let mut len = 0;
    for c in s {
        match c {
            b'+' => len += 1,
            b'-' => {
                if len > 0 {
                    len -= 1;
                    if non_sorted_at.is_some() && non_sorted_at.unwrap() >= len {
                        non_sorted_at = None;
                    }
                    if len < sorted_prefix {
                        sorted_prefix = len;
                    }
                }
            }
            b'1' => {
                if non_sorted_at.is_some() {
                    out_line!(false);
                    return;
                }
                sorted_prefix = len;
            }
            b'0' => {
                if sorted_prefix == len || len <= 1 {
                    out_line!(false);
                    return;
                }
                if non_sorted_at.is_none() {
                    non_sorted_at = Some(len - 1);
                }
            }
            _ => unreachable!(),
        }
    }
    out_line!(true);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i, &pre_calc);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
