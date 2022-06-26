//{"name":"B - 1D Pawn","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_b","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 5\n1 3 4\n3 3 1 1 2\n","output":"2 4 5\n"},{"input":"2 2 2\n1 2\n1 2\n","output":"1 2\n"},{"input":"10 6 9\n1 3 5 7 8 9\n1 2 3 4 5 6 5 6 2\n","output":"2 5 6 7 9 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1DPawn"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let q = input.read_usize();
    let mut a = input.read_usize_vec(k);

    a.push(n + 1);
    for _ in 0..q {
        let id = input.read_usize() - 1;
        if a[id] + 1 != a[id + 1] {
            a[id] += 1;
        }
    }
    a.pop();
    out_line!(a);
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
