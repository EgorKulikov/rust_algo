//{"name":"C - RANDOM","group":"AtCoder - TOYOTA SYSTEMS Programming Contest 2022(AtCoder Beginner Contest 279)","url":"https://atcoder.jp/contests/abc279/tasks/abc279_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n##.#\n##..\n#...\n.###\n..##\n...#\n","output":"Yes\n"},{"input":"3 3\n#.#\n.#.\n#.#\n##.\n##.\n.#.\n","output":"No\n"},{"input":"2 1\n#\n.\n#\n.\n","output":"Yes\n"},{"input":"8 7\n#..#..#\n.##.##.\n#..#..#\n.##.##.\n#..#..#\n.##.##.\n#..#..#\n.##.##.\n....###\n####...\n....###\n####...\n....###\n####...\n....###\n####...\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRANDOM"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let h = input.read_usize();
    let w = input.read_usize();
    let s = input.read_table::<char>(h, w);
    let t = input.read_table::<char>(h, w);

    let mut ss = (0..w)
        .into_iter()
        .map(|i| s.column(i).collect_vec())
        .collect_vec();
    ss.sort();
    let mut tt = (0..w)
        .into_iter()
        .map(|i| t.column(i).collect_vec())
        .collect_vec();
    tt.sort();
    set_bool_output(BoolOutput::YesNo);
    out_line!(ss == tt);
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
