//{"name":"B1. Basketbology (Decision)","group":"Codeforces - Abakoda 2021 Long Contest","url":"http://codeforces.com/gym/103496/problem/B1","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n10 15 20\n28 16 18\n","output":"YES\n"},{"input":"3\n10 15 20\n24 13 13\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1BasketbologyDecision"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let x = input.read_unsigned_vec(n);
    let mut c = input.read_unsigned_vec(n);

    c.sort_unstable();
    for (a, b) in x.into_iter().zip(c.into_iter()) {
        if a > b {
            out_line!("NO");
            return;
        }
    }
    out_line!("YES");
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
