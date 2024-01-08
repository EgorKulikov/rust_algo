//{"name":"A. 2023","group":"Codeforces - Good Bye 2023","url":"https://codeforces.com/contest/1916/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2 2\n5 2\n3 1\n7 17 7\n4 2\n1 289 1 1\n3 1\n7 17 17\n1 1\n289\n1 1\n2023\n1 3\n1\n","output":"NO\nNO\nYES\n7 1\nYES\n1\nYES\n7\nYES\n1\nYES\n7 17 17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A2023"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let b = input.read_int_vec(n);

    let mut p = 2023;
    for i in b {
        if p % i != 0 {
            out.print_line(false);
            return;
        }
        p /= i;
    }
    out.print_line(true);
    let mut ans = vec![p];
    for _ in 1..k {
        ans.push(1);
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
