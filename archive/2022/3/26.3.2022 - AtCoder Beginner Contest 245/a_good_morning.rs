//{"name":"A - Good morning","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_a","interactive":false,"timeLimit":2000,"tests":[{"input":"7 0 6 30\n","output":"Aoki\n"},{"input":"7 30 7 30\n","output":"Takahashi\n"},{"input":"0 0 23 59\n","output":"Takahashi\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGoodMorning"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let a = input.read_int();
    let b = input.read_int();
    let c = input.read_int();
    let d = input.read_int();

    if (a, b) <= (c, d) {
        out_line!("Takahashi");
    } else {
        out_line!("Aoki");
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
