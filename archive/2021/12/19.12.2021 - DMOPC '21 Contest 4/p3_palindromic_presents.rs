//{"name":"P3 - Palindromic Presents","group":"DMOJ - DMOPC '21 Contest 4","url":"https://dmoj.ca/problem/dmopc21c4p3","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3PalindromicPresents"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::{out, out_line};

fn solve(_input: &mut Input, _test_case: usize) {
    let ans = [
        0, 10001, 10101, 10201, 10301, 11011, 11111, 11211, 12021, 13031,
    ];
    for i in 0..1024 {
        let mut sum = 0;
        for j in 0..10 {
            if i.is_set(j) {
                sum += ans[j];
            }
        }
        let s = sum.to_string();
        let s = s.into_bytes();
        let mut t = s.clone();
        t.reverse();
        assert_eq!(s, t);
    }
    for i in ans {
        out_line!(i);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
