//{"name":"D - 250-like Number","group":"AtCoder - AtCoder Beginner Contest 250","url":"https://atcoder.jp/contests/abc250/tasks/abc250_d","interactive":false,"timeLimit":2000,"tests":[{"input":"250\n","output":"2\n"},{"input":"1\n","output":"0\n"},{"input":"123456789012345\n","output":"226863\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D250LikeNumber"}}}

use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::primes;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();

    let p = primes::<usize>(1000000);
    let mut ans = 0;
    for &i in &p {
        ans += p.lower_bound(&((n / i / i / i + 1).min(i)));
    }
    out_line!(ans);
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
