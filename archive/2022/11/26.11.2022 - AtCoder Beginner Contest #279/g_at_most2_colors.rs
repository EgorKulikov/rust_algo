//{"name":"G - At Most 2 Colors","group":"AtCoder - TOYOTA SYSTEMS Programming Contest 2022(AtCoder Beginner Contest 279)","url":"https://atcoder.jp/contests/abc279/tasks/abc279_g","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 3\n","output":"21\n"},{"input":"10 5 2\n","output":"1024\n"},{"input":"998 244 353\n","output":"952364159\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAtMost2Colors"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let c = input.read_int();

    if c == 1 {
        out_line!(1);
        return;
    }
    type Mod = ModIntF;
    let mut ans = vec![Mod::zero(); n];
    let mut sum = vec![Mod::zero(); n];
    ans[0] = Mod::zero();
    sum[0] = Mod::zero();
    for i in 1..n {
        ans[i] = if i < k - 1 {
            sum[i - 1] + Mod::one()
        } else {
            sum[i - 1] - sum[i + 1 - k] + sum[i + 1 - k] * Mod::new(c - 1) + Mod::one()
        };
        sum[i] = sum[i - 1] + ans[i];
    }
    let mut res = Mod::from(c);
    for i in 1..n {
        res += ans[n - i] * Mod::new(c) * Mod::new(c - 1);
    }
    out_line!(res);
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
