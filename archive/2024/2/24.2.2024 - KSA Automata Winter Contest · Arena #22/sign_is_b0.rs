//{"name":"Sign is B0","group":"Baekjoon Online Judge","url":"https://www.acmicpc.net/contest/problem/1238/7","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n1 1 1 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SignIsB0"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let n = input.read_long();

    let mut ans = 0;
    if b * n - c >= 0 {
        let right = (b * n - c) / (b + c);
        ans.maxim(right + 1);
        let mut left = 0;
        let mut right = right + 1;
        let f = |i: i64| -> i64 {
            if a * (n - i) >= c {
                i + (a * (n - i) - c) / (a + c) + 1
            } else {
                i
            }
        };
        while right - left > 5 {
            let mid_left = (2 * left + right) / 3;
            let mid_right = (left + 2 * right) / 3;
            let mid_left_val = f(mid_left);
            let mid_right_val = f(mid_right);
            if mid_left_val > mid_right_val {
                right = mid_right;
            } else {
                left = mid_left;
            }
        }
        for i in left..=right {
            ans.maxim(f(i));
        }
    }
    if a * n - c >= 0 {
        let left = (a * n - c) / (a + c);
        ans.maxim(left + 1);
        let mut right = left + 1;
        let mut left = 0;
        let f = |i: i64| -> i64 {
            if b * (n - i) >= c {
                i + (b * (n - i) - c) / (b + c) + 1
            } else {
                i
            }
        };
        while right - left > 5 {
            let mid_left = (2 * left + right) / 3;
            let mid_right = (left + 2 * right) / 3;
            let mid_left_val = f(mid_left);
            let mid_right_val = f(mid_right);
            if mid_left_val > mid_right_val {
                right = mid_right;
            } else {
                left = mid_left;
            }
        }
        for i in left..=right {
            ans.maxim(f(i));
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    if false {
        true
    } else {
        input.skip_whitespace();
        input.peek().is_none()
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
