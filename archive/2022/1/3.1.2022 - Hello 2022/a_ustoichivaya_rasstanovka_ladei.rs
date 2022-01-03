//{"name":"A. Устойчивая расстановка ладей","group":"Codeforces - Hello 2022","url":"https://codeforces.com/contest/1621/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 2\n3 3\n1 1\n5 2\n40 33\n","output":"..R\n...\nR..\n-1\nR\n.....\nR....\n.....\n....R\n.....\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AUstoichivayaRasstanovkaLadei"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    if 2 * k > n + 1 {
        out_line!(-1);
        return;
    }
    let ans = Arr2d::generate(n, n, |r, c| {
        if r == c && r < 2 * k && r % 2 == 0 {
            'R'
        } else {
            '.'
        }
    });
    output().print_table(&ans);
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
