//{"name":"C - NewFolder(1)","group":"AtCoder - AtCoder Beginner Contest 261","url":"https://atcoder.jp/contests/abc261/tasks/abc261_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nnewfile\nnewfile\nnewfolder\nnewfile\nnewfolder\n","output":"newfile\nnewfile(1)\nnewfolder\nnewfile(2)\nnewfolder(1)\n"},{"input":"11\na\na\na\na\na\na\na\na\na\na\na\n","output":"a\na(1)\na(2)\na(3)\na(4)\na(5)\na(6)\na(7)\na(8)\na(9)\na(10)\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNewFolder1"}}}

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();

    let mut qty: DefaultMap<_, usize> = DefaultMap::new();
    for _ in 0..n {
        let s = input.read_string();
        if qty[s.clone()] == 0 {
            out_line!(s);
        } else {
            out_line!(s.clone() + "(" + &qty[s.clone()].to_string() + ")");
        }
        qty[s.clone()] += 1;
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
