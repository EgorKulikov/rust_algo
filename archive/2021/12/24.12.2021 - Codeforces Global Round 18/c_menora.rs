//{"name":"C. Менора","group":"Codeforces - Codeforces Global Round 18","url":"https://codeforces.com/contest/1615/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n11010\n11010\n2\n01\n11\n3\n000\n101\n9\n100010111\n101101100\n9\n001011011\n011010101\n","output":"0\n1\n-1\n3\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMenora"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let a: Str = input.read();
    let b: Str = input.read();

    let a_ones = a.iter().filter(|c| *c == b'1').count();
    let b_ones = b.iter().filter(|c| *c == b'1').count();
    if a_ones != b_ones && a_ones + b_ones != n + 1 {
        out_line!(-1);
        return;
    }
    let mut ans = n + 1;
    if a_ones == b_ones {
        let neq = a.iter().zip(b.iter()).filter(|(c, d)| c != d).count();
        ans.minim(neq);
    }
    if a_ones + b_ones == n + 1 {
        let eq = a.iter().zip(b.iter()).filter(|(c, d)| c == d).count();
        ans.minim(eq);
    }
    out_line!(ans);
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
