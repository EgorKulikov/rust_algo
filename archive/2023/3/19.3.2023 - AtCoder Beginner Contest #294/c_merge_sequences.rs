//{"name":"C - Merge Sequences","group":"AtCoder - AtCoder Beginner Contest 294","url":"https://atcoder.jp/contests/abc294/tasks/abc294_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n3 14 15 92\n6 53 58\n","output":"1 3 4 7\n2 5 6\n"},{"input":"4 4\n1 2 3 4\n100 200 300 400\n","output":"1 2 3 4\n5 6 7 8\n"},{"input":"8 12\n3 4 10 15 17 18 22 30\n5 7 11 13 14 16 19 21 23 24 27 28\n","output":"1 2 5 9 11 12 15 20\n3 4 6 7 8 10 13 14 16 17 18 19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMergeSequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = VecDeque::from(input.read_int_vec(n));
    let mut b = VecDeque::from(input.read_int_vec(m));

    let mut ans_a = Vec::with_capacity(n);
    let mut ans_b = Vec::with_capacity(m);
    for i in 1..=n + m {
        if let Some(&av) = a.front() {
            if let Some(&bv) = b.front() {
                if av < bv {
                    ans_a.push(i);
                    a.pop_front();
                } else {
                    ans_b.push(i);
                    b.pop_front();
                }
            } else {
                ans_a.push(i);
                a.pop_front();
            }
        } else {
            ans_b.push(i);
            b.pop_front();
        }
    }
    out_line!(ans_a);
    out_line!(ans_b);
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
