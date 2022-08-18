//{"name":"B - Nice Grid","group":"AtCoder - freee Programming Contest 2022（AtCoder Beginner Contest 264）","url":"https://atcoder.jp/contests/abc264/tasks/abc264_b","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n","output":"black\n"},{"input":"4 5\n","output":"white\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNiceGrid"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    fn convert(i: usize) -> usize {
        i.min(16 - i)
    }

    let r = convert(input.read_usize());
    let c = convert(input.read_usize());

    if r.min(c) % 2 == 1 {
        out_line!("black");
    } else {
        out_line!("white");
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
