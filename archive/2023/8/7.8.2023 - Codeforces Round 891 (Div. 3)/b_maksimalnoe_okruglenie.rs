//{"name":"B. Максимальное округление","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n1\n5\n99\n913\n1980\n20444\n20445\n60947\n419860\n40862016542130810467\n","output":"1\n10\n100\n1000\n2000\n20444\n21000\n100000\n420000\n41000000000000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaksimalnoeOkruglenie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::StrReader;
use algo_lib::{out, out_line};

type PreCalc = ();

fn solve(input: &mut Input, _test_case: usize, _data: &PreCalc) {
    let mut s = input.read_str();

    let mut start = 0;
    for i in 0..s.len() {
        if s[i] <= b'3' {
            start = i + 1;
        }
        if s[i] >= b'5' {
            if start == 0 {
                out!(1);
                for _ in 0..s.len() {
                    out!(0);
                }
                out_line!();
                return;
            }
            s[start - 1] += 1;
            s[start..].fill(b'0');
            break;
        }
    }
    out_line!(s);
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
