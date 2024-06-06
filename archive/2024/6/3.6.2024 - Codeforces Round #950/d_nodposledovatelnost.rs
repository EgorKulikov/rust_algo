//{"name":"D. НОД-последовательность","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n6\n20 6 12 3 48 36\n4\n12 6 3 4\n3\n10 12 3\n5\n32 16 8 4 2\n5\n100 50 2 10 20\n4\n2 4 8 1\n10\n7 4 6 2 4 5 1 4 2 8\n7\n5 9 6 8 5 9 2\n6\n11 14 8 12 9 3\n9\n5 7 3 10 6 3 12 6 3\n3\n4 2 4\n8\n1 6 11 12 6 12 3 6\n","output":"YES\nNO\nYES\nNO\nYES\nYES\nNO\nYES\nYES\nYES\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNODPosledovatelnost"}}}

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::gcd;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    fn gcd_seq(a: &[i32]) -> Vec<i32> {
        let mut b = Vec::with_capacity(a.len() - 1);
        for (&i, &j) in a.consecutive_iter() {
            b.push(gcd(i, j));
        }
        b
    }
    let b = gcd_seq(&a);
    for i in 0..n - 2 {
        if b[i] > b[i + 1] {
            for j in 0..3 {
                let mut a = a.clone();
                a.remove(i + j);
                let b = gcd_seq(&a);
                let mut ok = true;
                for (&i, &j) in b.consecutive_iter() {
                    if i > j {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    out.print_line(true);
                    return;
                }
            }
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
}
//END MAIN
