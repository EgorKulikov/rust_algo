//{"name":"K. Divisible by three","group":"Codeforces - Replay of Ostad Presents Intra KUET Programming Contest 2023","url":"https://codeforces.com/gym/104663/problem/K","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n6 192021\n4 1234\n1 3\n2 34\n10 1234560070\n","output":"7\n4\n1\n1\n27\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KDivisibleByThree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let _m = input.read_size();
    let s = input.read_str();

    let mut ans = 0i64;
    let mut qty = [1, 0, 0];
    let mut sum = 0;
    for c in s {
        sum += (c - b'0') as usize;
        sum %= 3;
        ans += qty[sum];
        qty[sum] += 1;
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
