//{"name":"F. Скриншоты чата","group":"Codeforces - Codeforces Round 925 (Div. 3)","url":"https://codeforces.com/contest/1931/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n5 1\n1 2 3 4 5\n4 4\n1 2 3 4\n2 3 1 4\n3 2 1 4\n4 2 3 1\n6 2\n1 3 5 2 4 6\n6 3 5 2 1 4\n3 3\n1 2 3\n2 3 1\n3 2 1\n10 2\n1 2 3 4 5 6 7 8 9 10\n10 9 8 7 6 5 4 3 2 1\n1 1\n1\n5 2\n1 2 3 5 4\n2 1 3 5 4\n3 3\n3 1 2\n2 3 1\n1 3 2\n5 4\n3 5 1 4 2\n2 5 1 4 3\n1 5 4 3 2\n5 1 4 3 2\n3 3\n1 3 2\n2 1 3\n3 2 1\n","output":"YES\nYES\nYES\nYES\nNO\nYES\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSkrinshotiChata"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_size_table(k, n);

    if k == 1 {
        out.print_line(true);
        return;
    }
    let mut order = Vec::with_capacity(n);
    let mut switch = None;
    let mut i = 1;
    let mut j = 1;
    while order.len() < n {
        if i < n && a[(0, i)] == a[(1, 0)] {
            if j < n && a[(1, j)] == a[(0, 0)] {
                switch = Some(order.len());
                order.push(a[(0, i)]);
                order.push(a[(1, j)]);
                i += 1;
                j += 1;
            } else {
                order.push(a[(0, i)]);
                i += 1;
            }
        } else if j < n && a[(1, j)] == a[(0, 0)] {
            order.push(a[(1, j)]);
            j += 1;
        } else {
            if i == n || j == n || a[(0, i)] != a[(1, j)] {
                out.print_line(false);
                return;
            }
            order.push(a[(0, i)]);
            i += 1;
            j += 1;
        }
    }
    fn check(a: &Arr2d<usize>, order: &[usize]) -> bool {
        for i in 2..a.d1() {
            let mut at = 0;
            for &j in a.row(i).skip(1) {
                if order[at] == a[(i, 0)] {
                    at += 1;
                }
                if order[at] != j {
                    return false;
                }
                at += 1;
            }
        }
        true
    }
    if check(&a, &order) {
        out.print_line(true);
    } else if let Some(sw) = switch {
        order.swap(sw, sw + 1);
        if check(&a, &order) {
            out.print_line(true);
        } else {
            out.print_line(false);
        }
    } else {
        out.print_line(false);
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
    //    tester::stress_test();
}
//END MAIN
