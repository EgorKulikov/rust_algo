//{"name":"WOSS Dual Olympiad 2021 J2/S1: Secret Formula","group":"DMOJ","url":"https://dmoj.ca/problem/wossoly2021j2s1","interactive":false,"timeLimit":2000,"tests":[{"input":"4 1 C H N Y X M 2 O\n10\n2 1 0 3 1 0 4 8 9 0\n","output":"C14H14N2O4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"WOSSDualOlympiad2021J2S1SecretFormula"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let c: Vec<char> = input.read_vec(10);
    let n = input.read();
    let s: Vec<usize> = input.read_vec(n);

    let ans = s.into_iter().map(|i| c[i]).collect::<String>();
    out_line!(ans);
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
