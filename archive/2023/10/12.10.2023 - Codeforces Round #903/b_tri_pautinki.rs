//{"name":"B. Три паутинки","group":"Codeforces - Codeforces Round 903 (Div. 3)","url":"https://codeforces.com/contest/1881/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"15\n1 3 2\n5 5 5\n6 36 12\n7 8 7\n6 3 3\n4 4 12\n12 6 8\n1000000000 1000000000 1000000000\n3 7 1\n9 9 1\n9 3 6\n2 8 2\n5 3 10\n8 4 8\n2 8 4\n","output":"YES\nYES\nNO\nNO\nYES\nYES\nNO\nYES\nNO\nNO\nYES\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTriPautinki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let len = input.read_long_vec(3);

    let s = len.iter().sum::<i64>();
    for i in 3..=6 {
        if s % i != 0 {
            continue;
        }
        let mut good = true;
        for &j in &len {
            if j % (s / i) != 0 {
                good = false;
                break;
            }
        }
        if good {
            out.print_line(true);
            return;
        }
    }
    out.print_line(false);
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
            for i in 0usize..t {
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
