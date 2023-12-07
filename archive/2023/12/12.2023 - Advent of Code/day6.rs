//{"name":"day6","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"day6"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use std::iter::once;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    input.read_str();
    let time: Vec<i64> = input.read_line().parse_vec();
    input.read_str();
    let dist: Vec<i64> = input.read_line().parse_vec();

    fn convert(arr: &[i64]) -> i64 {
        let mut res = 0;
        for &i in arr {
            let mut ten = 1;
            while ten <= i {
                ten *= 10;
            }
            res = res * ten + i;
        }
        res
    }

    fn solve(t: &[i64], d: &[i64]) -> i64 {
        let mut ans = 1;
        for (&t, &d) in t.iter().zip(d.iter()) {
            let mut left = 0;
            let mut right = t / 2;
            while left < right {
                let mid = (left + right) / 2;
                if mid * (t - mid) > d {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            ans *= t - 2 * left + 1;
        }
    }

    {
        // part 1
        out.print_line(solve(&time, &dist));
    }

    {
        // part 2
        out.print_line(solve(&[convert(&time)], &[convert(&dist)]));
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
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
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
