//{"name":"A - Three Integers","group":"AtCoder - AtCoder Regular Contest 143","url":"https://atcoder.jp/contests/arc143/tasks/arc143_a","interactive":false,"timeLimit":2000,"tests":[{"input":"2 2 3\n","output":"3\n"},{"input":"0 0 1\n","output":"-1\n"},{"input":"0 0 0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AThreeIntegers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let mut a = input.read_long_vec(3);

    a.sort();
    if a[2] > a[0] + a[1] {
        out_line!(-1);
        return;
    }
    let mut ans = 0;
    ans += a[1] - a[0];
    a[1] -= ans;
    a[2] -= ans;
    ans += (a[2] - a[1]) * 2;
    ans += a[1] - (a[2] - a[1]);
    out_line!(ans);
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
