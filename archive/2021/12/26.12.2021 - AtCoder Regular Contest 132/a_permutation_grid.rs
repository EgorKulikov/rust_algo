//{"name":"A - Permutation Grid","group":"AtCoder - AtCoder Regular Contest 132","url":"https://atcoder.jp/contests/arc132/tasks/arc132_a","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 2 3 4 1\n4 2 3 1 5\n7\n1 5\n5 1\n1 1\n2 2\n3 3\n4 4\n5 5\n","output":"#.#.#.#\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APermutationGrid"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let r = input.read_vec::<usize>(n);
    let c = input.read_vec::<usize>(n);
    let q = input.read();

    for _ in 0..q {
        let cr = input.read::<usize>() - 1;
        let cc = input.read::<usize>() - 1;

        if r[cr] + c[cc] > n {
            out!('#');
        } else {
            out!('.');
        }
    }
    out_line!();
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
