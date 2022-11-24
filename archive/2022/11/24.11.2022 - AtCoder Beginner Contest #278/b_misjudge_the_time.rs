//{"name":"B - Misjudge the Time","group":"AtCoder - AtCoder Beginner Contest 278","url":"https://atcoder.jp/contests/abc278/tasks/abc278_b","interactive":false,"timeLimit":2000,"tests":[{"input":"1 23\n","output":"1 23\n"},{"input":"19 57\n","output":"20 0\n"},{"input":"20 40\n","output":"21 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMisjudgeTheTime"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let mut h = input.read_usize();
    let mut m = input.read_usize();

    loop {
        let nh = h / 10 * 10 + m / 10;
        let nm = h % 10 * 10 + m % 10;
        if nh < 24 && nm < 60 {
            out_line!(h, m);
            return;
        }
        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
            if h == 24 {
                h = 0;
            }
        }
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
