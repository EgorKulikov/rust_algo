//{"name":"E - 2xN Grid","group":"AtCoder - AtCoder Beginner Contest 294","url":"https://atcoder.jp/contests/abc294/tasks/abc294_e","interactive":false,"timeLimit":2000,"tests":[{"input":"8 4 3\n1 2\n3 2\n2 3\n3 1\n1 4\n2 1\n3 3\n","output":"4\n"},{"input":"10000000000 1 1\n1 10000000000\n1 10000000000\n","output":"10000000000\n"},{"input":"1000 4 7\n19 79\n33 463\n19 178\n33 280\n19 255\n33 92\n34 25\n19 96\n12 11\n19 490\n33 31\n","output":"380\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2xNGrid"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input) {
    let _l = input.read_size();
    let n1 = input.read_size();
    let n2 = input.read_size();
    let mut a1 = VecDeque::from(input.read_size_pair_vec(n1));
    let mut a2 = VecDeque::from(input.read_size_pair_vec(n2));

    let mut ans = 0;
    let mut v1 = 0;
    let mut l1 = 0;
    let mut v2 = 0;
    let mut l2 = 0;
    while !a1.is_empty() || !a2.is_empty() {
        if l1 == 0 {
            let (v, l) = a1.pop_front().unwrap();
            v1 = v;
            l1 = l;
        }
        if l2 == 0 {
            let (v, l) = a2.pop_front().unwrap();
            v2 = v;
            l2 = l;
        }
        let cur = l1.min(l2);
        if v1 == v2 {
            ans += cur;
        }
        l1 -= cur;
        l2 -= cur;
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
