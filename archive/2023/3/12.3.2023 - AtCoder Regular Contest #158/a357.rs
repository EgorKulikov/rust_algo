//{"name":"A - +3 +5 +7","group":"AtCoder - AtCoder Regular Contest 158","url":"https://atcoder.jp/contests/arc158/tasks/arc158_a","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 8 8\n1 1 1\n5 5 10\n10 100 1000\n","output":"2\n0\n-1\n315\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A357"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let mut x = input.read_size_vec(3);

    x.sort();
    for &i in &x {
        if i % 2 != x[0] % 2 {
            out_line!(-1);
            return;
        }
    }
    let sum = x[0] + x[1] + x[2];
    if sum % 3 != 0 {
        out_line!(-1);
        return;
    }
    let mut d1 = (x[2] - x[0]) / 2;
    let d2 = (x[2] - x[1]) / 2;
    if d2 * 2 >= d1 {
        out_line!((d1 + d2) / 3);
        return;
    }
    let mut ans = d2;
    d1 -= d2 * 2;
    ans += d1 / 3 * 2;
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read_size();
    for _ in 0..t {
        solve(&mut input);
    }
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
