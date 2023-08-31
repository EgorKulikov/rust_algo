//{"name":"B. Две бинарных строки","group":"Codeforces - Educational Codeforces Round 154 (Rated for Div. 2)","url":"https://codeforces.com/contest/1861/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n01010001\n01110101\n01001\n01001\n000101\n010111\n00001\n01111\n011\n001\n001001\n011011\n010001\n011011\n","output":"YES\nYES\nYES\nNO\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDveBinarnikhStroki"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let a = input.read_str();
    let b = input.read_str();

    for ((&a0, &a1), (&b0, &b1)) in a.consecutive_iter().zip(b.consecutive_iter()) {
        if a0 == b'0' && a1 == b'1' && b0 == b'0' && b1 == b'1' {
            out_line!(true);
            return;
        }
    }
    out_line!(false);
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
