//{"name":"E. Хорошие тройки","group":"Codeforces - Codeforces Round 913 (Div. 3)","url":"https://codeforces.com/contest/1907/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n11\n0\n1\n2\n3\n4\n5\n3141\n999\n2718\n9999999\n10000000\n","output":"9\n1\n3\n6\n10\n15\n21\n1350\n166375\n29160\n1522435234375\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKhoroshieTroiki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_size();

    let mut ans = 1;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        ans *= (d + 2) * (d + 1) / 2;
    }
    out.print_line(ans);
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
