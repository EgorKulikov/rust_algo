//{"name":"C. Удаление некрасивых пар","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n4\naabc\n5\nabaca\n10\navbvvcvvvd\n7\nabcdefg\n5\ndabbb\n8\naacebeaa\n7\nbbbbacc\n6\ndacfcc\n6\nfdfcdc\n9\ndbdcfbbdc\n","output":"0\n1\n2\n1\n1\n0\n1\n0\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUdalenieNekrasivikhPar"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut qty = [0usize; 26];
    for c in s {
        qty[(c - b'a') as usize] += 1;
    }
    for i in qty {
        if 2 * i > n {
            out.print_line(2 * i - n);
            return;
        }
    }
    out.print_line(n % 2);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
