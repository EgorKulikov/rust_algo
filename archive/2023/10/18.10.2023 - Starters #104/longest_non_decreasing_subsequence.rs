//{"name":"Longest Non Decreasing Subsequence","group":"CodeChef - START104A","url":"https://www.codechef.com/START104A/problems/LNDS","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n11\n2\n10\n5\n01110\n1\n0\n","output":"4\n3\n31\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LongestNonDecreasingSubsequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut ans = 0;
    let mut delta = n;
    let mut last = vec![0; 2 * n];
    let mut sum = 0;
    for (i, c) in s.into_iter().enumerate() {
        if c == b'1' {
            sum += i + 1;
            last[delta] = i + 1;
            delta += 1;
        } else {
            delta -= 1;
            sum += i + 1 - last[delta];
        }
        ans += sum;
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
