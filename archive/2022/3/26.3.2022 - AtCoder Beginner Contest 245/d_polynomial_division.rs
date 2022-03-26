//{"name":"D - Polynomial division","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_d","interactive":false,"timeLimit":2000,"tests":[{"input":"1 2\n2 1\n12 14 8 2\n","output":"6 4 2\n"},{"input":"1 1\n100 1\n10000 0 -1\n","output":"100 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPolynomialDivision"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_int_vec(n + 1);
    let c = input.read_int_vec(n + m + 1);

    let mut b = vec![0; m + 1];
    for i in (0..=m).rev() {
        let mut res = c[n + i];
        for j in (n + i).saturating_sub(m)..n {
            res -= a[j] * b[n + i - j];
        }
        b[i] = res / a[n];
    }
    out_line!(b);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
