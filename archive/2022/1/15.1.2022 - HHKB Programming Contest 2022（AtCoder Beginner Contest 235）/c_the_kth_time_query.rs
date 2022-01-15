//{"name":"C - The Kth Time Query","group":"AtCoder - HHKB Programming Contest 2022（AtCoder Beginner Contest 235）","url":"https://atcoder.jp/contests/abc235/tasks/abc235_c","interactive":false,"timeLimit":3000,"tests":[{"input":"6 8\n1 1 2 3 1 2\n1 1\n1 2\n1 3\n1 4\n2 1\n2 2\n2 3\n4 1\n","output":"1\n2\n5\n-1\n3\n6\n-1\n-1\n"},{"input":"3 2\n0 1000000000 999999999\n1000000000 1\n123456789 1\n","output":"2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheKthTimeQuery"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let q = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let mut pos: DefaultMap<u32, Vec<usize>> = DefaultMap::new();
    for (i, j) in a.into_iter().enumerate() {
        pos[j].push(i + 1);
    }
    for _ in 0..q {
        let x = input.read_unsigned();
        let k = input.read_usize() - 1;
        let v = &pos[x];
        out_line!(v.get(k).cloned());
    }
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
