//{"name":"#3 - AN2DL","group":"DMOJ - COCI '23 Contest 1","url":"https://dmoj.ca/problem/coci23c1p3","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3\n1 1 2\n2 3 4\n4 3 2\n3 3\n","output":"4\n"},{"input":"3 3\n1 1 2\n2 3 4\n4 3 2\n2 1\n","output":"2 3 4\n4 3 4\n"},{"input":"5 5\n-1 -3 -4 -2 4\n-8 -7 -9 -10 11\n5 2 -8 -2 1\n13 -3 -2 -6 -9\n11 6 2 7 4\n2 3\n","output":"-1 -2 11\n5 2 11\n13 2 1\n13 7 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AN2DL"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::sliding_window::SlidingWindow;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_table(n, m);
    let r = input.read_size();
    let s = input.read_size();

    let mut t1 = Arr2d::new(n, m - s + 1, 0);
    for i in 0..n {
        let mut sw = SlidingWindow::new(s, i32::cmp);
        for j in 0..s - 1 {
            sw.push(a[(i, j)]);
        }
        for j in s - 1..m {
            sw.push(a[(i, j)]);
            t1[(i, j + 1 - s)] = *sw.get().unwrap();
        }
    }
    let mut ans = Arr2d::new(n - r + 1, m - s + 1, 0);
    for i in t1.cols() {
        let mut sw = SlidingWindow::new(r, i32::cmp);
        for j in 0..r - 1 {
            sw.push(t1[(j, i)]);
        }
        for j in r - 1..n {
            sw.push(t1[(j, i)]);
            ans[(j + 1 - r, i)] = *sw.get().unwrap();
        }
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
