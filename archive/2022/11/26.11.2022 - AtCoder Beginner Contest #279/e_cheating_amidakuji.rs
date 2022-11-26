//{"name":"E - Cheating Amidakuji","group":"AtCoder - TOYOTA SYSTEMS Programming Contest 2022(AtCoder Beginner Contest 279)","url":"https://atcoder.jp/contests/abc279/tasks/abc279_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n1 2 3 2\n","output":"1\n3\n2\n4\n"},{"input":"3 3\n2 2 2\n","output":"1\n1\n1\n"},{"input":"10 10\n1 1 1 9 4 4 2 1 3 3\n","output":"2\n2\n2\n3\n3\n3\n1\n3\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECheatingAmidakuji"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::when;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_usize_vec(m).dec_by_one();

    let mut cur = 0usize;
    let mut pos_one = Vec::with_capacity(m);
    for &i in &a {
        pos_one.push(cur);
        when! {
            i == cur => cur += 1,
            i + 1 == cur => cur -= 1,
            else => {},
        }
    }
    let mut ans = Vec::with_capacity(m);
    let mut p = (0..n).collect_vec();
    for (a, pos_one) in (a.into_iter().zip(pos_one.into_iter())).rev() {
        ans.push(p[pos_one] + 1);
        p.swap(a, a + 1);
    }
    ans.reverse();
    output().print_per_line(&ans);
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
