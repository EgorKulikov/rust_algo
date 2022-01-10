//{"name":"#5 - Kućice","group":"DMOJ - COCI '21 Contest 3","url":"https://dmoj.ca/problem/coci21c3p5","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n5 5\n","output":"1\n"},{"input":"3\n-1 -1\n1 -1\n0 1\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Kuice"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::mod_int::{BaseModInt, ModInt7};
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::number_ext::Power;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();

    type Mod = ModInt7;
    let ans = Mod::new(2).power(n - 1) * Mod::from_index(n);
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
