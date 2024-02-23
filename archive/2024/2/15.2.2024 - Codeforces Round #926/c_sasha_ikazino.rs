//{"name":"C. Саша и казино","group":"Codeforces - Codeforces Round 926 (Div. 2)","url":"https://codeforces.com/contest/1929/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n2 1 7\n2 1 1\n2 3 15\n3 3 6\n4 4 5\n5 4 7\n4 88 1000000000\n25 69 231\n13 97 18806\n","output":"YES\nNO\nYES\nNO\nNO\nYES\nNO\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSashaIKazino"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let k = input.read_long();
    let x = input.read_long();
    let a = input.read_long();

    let mut lost = 0;
    for _ in 0..=x {
        let bet = (lost + 1).upper_div(k - 1);
        lost += bet;
        if lost > a {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
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
