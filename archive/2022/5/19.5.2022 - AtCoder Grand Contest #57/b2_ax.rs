//{"name":"B - 2A + x","group":"AtCoder - AtCoder Grand Contest 057","url":"https://atcoder.jp/contests/agc057/tasks/agc057_b","interactive":false,"timeLimit":4000,"tests":[{"input":"4 2\n5 8 12 20\n","output":"6\n"},{"input":"4 5\n24 25 26 27\n","output":"0\n"},{"input":"4 1\n24 25 26 27\n","output":"3\n"},{"input":"10 5\n39 23 3 7 16 19 40 16 33 6\n","output":"13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B2AX"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let x = input.read_long();
    let a = input.read_long_vec(n);

    let mut options = Vec::with_capacity(n);
    let target = *a.iter().max().unwrap();
    for mut i in a {
        let mut add = 0;
        while 2 * i <= target {
            i *= 2;
            add *= 2;
            add += x;
        }
        if i + add < target {
            options.push((i + add, 2 * i));
        }
    }
    if options.is_empty() {
        out_line!(0);
        return;
    }
    let mut max = target;
    options.sort_unstable();
    let mut ans = None;
    for (l, h) in options {
        ans.minim(max - l);
        max.maxim(h);
    }
    ans.minim(max - target);
    let ans = ans.unwrap();
    out_line!(if ans < x { 0 } else { ans });
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
