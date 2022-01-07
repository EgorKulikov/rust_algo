//{"name":"H. Halcon Collider","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/H","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n1 1 1 1\n","output":"1.414213562373\n0\nTR\n"},{"input":"3\n1 2 1 1\n1 3 2 4\n4 3 5 2\n","output":"2.828427124746\n1\nTL\n6.708203932499\n3\nBR\n64.621977685614\n21\nBR\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHalconCollider"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::{gcd, lcm};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let w = input.read_unsigned();
    let h = input.read_unsigned();
    let mut x = input.read_unsigned();
    let mut y = input.read_unsigned();

    let g = gcd(x, y);
    x /= g;
    y /= g;
    let n = lcm(w / gcd(w, x), h / gcd(h, y));
    let a = n * x / w;
    let b = n * y / h;
    out_line!(f64::hypot((n * x) as f64, (n * y) as f64));
    out_line!(a + b - 2);
    out!(if b % 2 == 0 { 'B' } else { 'T' });
    out!(if a % 2 == 0 { 'L' } else { 'R' });
    out_line!();
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
