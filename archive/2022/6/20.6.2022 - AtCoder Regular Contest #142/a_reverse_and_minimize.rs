//{"name":"A - Reverse and Minimize","group":"AtCoder - AtCoder Regular Contest 142","url":"https://atcoder.jp/contests/arc142/tasks/arc142_a","interactive":false,"timeLimit":2000,"tests":[{"input":"1420 142\n","output":"3\n"},{"input":"1419 142\n","output":"2\n"},{"input":"6 19\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AReverseAndMinimize"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let mut k = input.read_usize();

    let mut rev = 0;
    let mut c = k;
    while c > 0 {
        rev *= 10;
        rev += c % 10;
        c /= 10;
    }
    if k > rev {
        out_line!(0);
        return;
    }
    let mut ans = 0;
    if rev != k {
        while rev <= n {
            ans += 1;
            rev *= 10;
        }
    }
    while k <= n {
        ans += 1;
        k *= 10;
    }
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
