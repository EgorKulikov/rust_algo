//{"name":"A. Разбиваем лагерь","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2 3\n1 4 1\n1 4 2\n1 1 1\n1 3 2\n19 7 18\n0 0 0\n7 0 0\n0 24 0\n1000000000 1000000000 1000000000\n","output":"3\n-1\n3\n-1\n3\n28\n0\n7\n8\n1666666667\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARazbivaemLager"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();

    let ans = a + (c + b).upper_div(3);
    if b % 3 != 0 && b % 3 + c < 3 {
        out.print_line(-1);
    } else {
        out.print_line(ans);
    }
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
