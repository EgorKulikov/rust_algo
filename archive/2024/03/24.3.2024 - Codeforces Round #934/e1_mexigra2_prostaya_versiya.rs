//{"name":"E1. MEX игра - 2 (простая версия)","group":"Codeforces - Codeforces Round 934 (Div. 1)","url":"https://codeforces.com/contest/1943/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 4\n4 5\n2 1000000000\n1000000000 1000000000 1000000000\n3 2\n2 3 100 1\n1 1\n2 2\n3 1\n1 1 1 1\n","output":"2\n1\n3\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1MEXIgra2ProstayaVersiya"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let m = input.read_size();
    let k = input.read_long();
    let f = input.read_long_vec(m + 1);

    let mut left = 0;
    let mut right = m + 1;
    while left < right {
        let mid = (left + right + 1) / 2;
        let f = f[..mid].to_vec().sorted();
        let mut rec = RecursiveFunction::new(|rec, f: Vec<i64>| -> bool {
            let mut f = f[1..].to_vec();
            if f.is_empty() {
                return false;
            }
            if f[0] <= k {
                return true;
            }
            let i = f.len() - 1;
            let mut rem = k;
            for j in (0..=i).rev() {
                let max_delta = f[j] - if j == 0 { 0 } else { f[j - 1] };
                let mut by = (i + 1 - j) as i64;
                if rem >= max_delta * by {
                    for k in j..=i {
                        f[k] -= max_delta;
                    }
                    rem -= max_delta * by;
                } else {
                    for k in (j..=i).rev() {
                        let cur = rem / by;
                        f[k] -= cur;
                        rem -= cur;
                        by -= 1;
                    }
                    break;
                }
            }
            if rec.call(f) {
                return true;
            }
            false
        });
        let mut can = false;
        for i in 1..=f.len() {
            let f = f[..i].to_vec();
            if rec.call(f) {
                can = true;
                break;
            }
        }
        if can {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    out.print_line(left);
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
