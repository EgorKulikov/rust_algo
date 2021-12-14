//{"name":"C. Покраска массива","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n1 2 3 4 5\n3\n10 5 15\n3\n100 10 200\n10\n9 8 2 6 6 2 8 6 5 4\n2\n1 3\n","output":"2\n0\n100\n0\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPokraskaMassiva"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<u64> = input.read_vec(n);

    for i in 0..2 {
        let mut g = 0u64;
        for i in (i..n).step_by(2) {
            g = gcd(g, a[i]);
        }
        let mut good = true;
        for i in ((1 - i)..n).step_by(2) {
            if a[i] % g == 0 {
                good = false;
                break;
            }
        }
        if good {
            out_line!(g);
            return;
        }
    }
    out_line!(0);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
