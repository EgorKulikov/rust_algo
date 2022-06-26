//{"name":"B - Counting Grids","group":"AtCoder - AtCoder Regular Contest 143","url":"https://atcoder.jp/contests/arc143/tasks/arc143_b","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n","output":"8\n"},{"input":"5\n","output":"704332752\n"},{"input":"100\n","output":"927703658\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCountingGrids"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();

    type Mod = ModIntF;
    let c: Combinations<Mod> = Combinations::new(n * n + 1);
    let mut ans = Mod::zero();
    for i in 0..n * n {
        ans += c.c(i, n - 1) * c.c(n * n - i - 1, n - 1);
    }
    ans *= c.fact(n - 1) * c.fact(n - 1) * c.fact(n * n + 1 - 2 * n);
    ans *= Mod::new(n.into_i32());
    ans *= Mod::new(n.into_i32());
    ans = c.fact(n * n) - ans;
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
