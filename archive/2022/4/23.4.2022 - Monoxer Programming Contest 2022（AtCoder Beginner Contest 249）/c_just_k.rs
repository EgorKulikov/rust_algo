//{"name":"C - Just K","group":"AtCoder - Monoxer Programming Contest 2022（AtCoder Beginner Contest 249）","url":"https://atcoder.jp/contests/abc249/tasks/abc249_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\nabi\naef\nbc\nacg\n","output":"3\n"},{"input":"2 2\na\nb\n","output":"0\n"},{"input":"5 2\nabpqxyz\naz\npq\nbc\ncy\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CJustK"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let k = input.read_usize();
    let s: Vec<Str> = input.read_vec(n);

    let mut ans = 0;
    for i in 1..(1 << n) {
        let mut cur = 0;
        for c in b'a'..=b'z' {
            let mut num = 0;
            for j in 0..n {
                if i.is_set(j) && s[j].as_slice().contains(&c) {
                    num += 1;
                }
            }
            if num == k {
                cur += 1;
            }
        }
        ans.maxim(cur);
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
