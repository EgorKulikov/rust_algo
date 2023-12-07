//{"name":"A. Ладья","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"1\nd5\n","output":"d1\nd2\nb5\ng5\nh5\nd3\ne5\nf5\nd8\na5\nd6\nd7\nc5\nd4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALadya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let row = input.read_char();
    let col = input.read_char();

    for r in 'a'..='h' {
        if r != row {
            out.print_line(format!("{}{}", r, col));
        }
    }
    for c in '1'..='8' {
        if c != col {
            out.print_line(format!("{}{}", row, c));
        }
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
