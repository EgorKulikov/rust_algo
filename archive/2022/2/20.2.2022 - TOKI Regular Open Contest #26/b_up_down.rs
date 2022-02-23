//{"name":"B. Up & Down","group":"TLX - TOKI Regular Open Contest #26","url":"https://tlx.toki.id/contests/troc-26/problems/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2 4\n1 0 0 0\n0 0 -1 0\n","output":"YES\n1\n"},{"input":"2 3\n3 2 -1\n0 0 0\n","output":"NO\n"},{"input":"3 3\n0 0 0\n0 -1 0\n0 0 0\n","output":"YES\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BUpDown"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_table::<i64>(n, m);

    let mut odd = 0;
    let mut even = 0;
    let mut unknown_odd = false;
    for i in 0..n {
        for j in 0..m {
            if (i + j) % 2 == 0 {
                if a[(i, j)] == -1 {
                    unknown_odd = false;
                } else {
                    even += a[(i, j)];
                }
            } else {
                if a[(i, j)] == -1 {
                    unknown_odd = true;
                } else {
                    odd += a[(i, j)];
                }
            }
        }
    }
    if unknown_odd {
        swap(&mut odd, &mut even);
    }
    if even > odd {
        out_line!("NO");
    } else {
        out_line!("YES");
        out_line!(odd - even);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
