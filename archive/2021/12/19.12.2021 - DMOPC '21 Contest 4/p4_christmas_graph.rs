//{"name":"P4 - Christmas Graph","group":"DMOJ - DMOPC '21 Contest 4","url":"https://dmoj.ca/problem/dmopc21c4p4","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n","output":"370370374\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4ChristmasGraph"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt7};
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();

    type Mod = ModInt7;
    let mut ans = Mod::zero();
    let c: Combinations<Mod> = Combinations::new(n + 1);
    let mut pw = Mod::new((n as i32) - 1);
    for i in 2..=n {
        pw *= Mod::new((n as i32) - 1);
        ans += c.c(n, i) * c.fact(i - 1) / pw;
    }
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
