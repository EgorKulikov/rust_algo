//{"name":"F. Matrix multiplication","group":"CPython.uz - CPython Programming Contest #3","url":"https://cpython.uz/competitions/contests/contest/326/problem/F","interactive":false,"timeLimit":800,"tests":[{"input":"39 9 99 9 99 9 90 0 00 0 00 0 0\n","output":"0 0 00 0 00 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMatrixMultiplication"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_table(n, n);
    let b = input.read_int_table(n, n);

    let mut c = Vec::with_capacity(n);
    for i in 0..n {
        let mut cur = Vec::new();
        for (j, &x) in b.column(i).enumerate() {
            if x == 1 {
                cur.push(j);
            }
        }
        c.push(cur);
    }
    let ans = Arr2d::generate(n, n, |i, j| {
        let mut res = 0;
        for &k in &c[j] {
            res += a[(i, k)];
        }
        res
    });
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
    //    tester::stress_test(run, tester::check);
}
//END MAIN
