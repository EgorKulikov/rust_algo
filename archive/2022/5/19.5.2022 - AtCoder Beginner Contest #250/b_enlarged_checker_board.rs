//{"name":"B - Enlarged Checker Board","group":"AtCoder - AtCoder Beginner Contest 250","url":"https://atcoder.jp/contests/abc250/tasks/abc250_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3 2\n","output":"..##..##\n..##..##\n..##..##\n##..##..\n##..##..\n##..##..\n..##..##\n..##..##\n..##..##\n##..##..\n##..##..\n##..##..\n"},{"input":"5 1 5\n","output":".....#####.....#####.....\n#####.....#####.....#####\n.....#####.....#####.....\n#####.....#####.....#####\n.....#####.....#####.....\n"},{"input":"4 4 1\n","output":".#.#\n.#.#\n.#.#\n.#.#\n#.#.\n#.#.\n#.#.\n#.#.\n.#.#\n.#.#\n.#.#\n.#.#\n#.#.\n#.#.\n#.#.\n#.#.\n"},{"input":"1 4 4\n","output":"....\n....\n....\n....\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BEnlargedCheckerBoard"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let a = input.read_usize();
    let b = input.read_usize();

    let ans = Arr2d::generate(
        n * a,
        n * b,
        |i, j| if i / a % 2 == j / b % 2 { '.' } else { '#' },
    );
    output().print_table(&ans);
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
