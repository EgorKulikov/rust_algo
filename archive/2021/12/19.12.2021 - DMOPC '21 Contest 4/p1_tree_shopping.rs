//{"name":"P1 - Tree Shopping","group":"DMOJ - DMOPC '21 Contest 4","url":"https://dmoj.ca/problem/dmopc21c4p1","interactive":false,"timeLimit":2000,"tests":[{"input":"2 5\n-2 -3 -2 3 7 -3\n-4 1 -4 -6 3 1\n2 2\n-4 1\n1 -2\n-2 -1\n3 -5\n","output":"0\n1\n1\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P1TreeShopping"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let q = input.read();
    let trees: Vec<(i64, i64, i64, i64, i64, i64)> = input.read_vec(n);

    for _ in 0..q {
        let mut ans = 0;
        let x: i64 = input.read();
        let y: i64 = input.read();
        for (x1, y1, x2, y2, x3, y3) in trees.iter().cloned() {
            let a1 = y1 - y2;
            let b1 = x2 - x1;
            let c1 = -a1 * x1 - b1 * y1;
            let a2 = y2 - y3;
            let b2 = x3 - x2;
            let c2 = -a2 * x2 - b2 * y2;
            let a3 = y3 - y1;
            let b3 = x1 - x3;
            let c3 = -a3 * x3 - b3 * y3;
            let mut has_pos = false;
            let mut has_neg = false;
            let v1 = a1 * x + b1 * y + c1;
            if v1 > 0 {
                has_pos = true;
            } else if v1 < 0 {
                has_neg = true;
            }
            let v2 = a2 * x + b2 * y + c2;
            if v2 > 0 {
                has_pos = true;
            } else if v2 < 0 {
                has_neg = true;
            }
            let v3 = a3 * x + b3 * y + c3;
            if v3 > 0 {
                has_pos = true;
            } else if v3 < 0 {
                has_neg = true;
            }
            if !has_pos || !has_neg {
                ans += 1;
            }
        }
        out_line!(ans);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
