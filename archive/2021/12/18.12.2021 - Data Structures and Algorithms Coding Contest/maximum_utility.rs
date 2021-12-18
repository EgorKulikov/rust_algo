//{"name":"Maximum utility","group":"HackerEarth - Data Structures and Algorithms Coding Contest","url":"https://www.hackerearth.com/challenges/competitive/data-structures-and-algorithms-coding-contest-dec/algorithm/max-utility-32996da6/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4 11\n11 8 15 4\n10 14 5 9\n5 12\n0 11 11 2 5\n10 15 9 12 12\n","output":"24\n46\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaximumUtility"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k: u32 = input.read();
    let a: Vec<u32> = input.read_vec(n);
    let b: Vec<u64> = input.read_vec(n);

    let mut ans = 0;
    for (a, b) in a.iter().zip(b.iter()) {
        if (*a & k) == *a {
            ans += *b;
        }
    }
    let mut prefix = 0u32;
    for i in (0usize..30).rev() {
        if !k.is_set(i) {
            prefix <<= 1;
            continue;
        }
        let mut cand: u64 = 0;
        for (a, b) in a.iter().zip(b.iter()) {
            let p = *a >> (i + 1);
            if (p & prefix) == p && !a.is_set(i) {
                cand += *b;
            }
        }
        ans.maxim(cand);
        prefix <<= 1;
        prefix += 1;
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
