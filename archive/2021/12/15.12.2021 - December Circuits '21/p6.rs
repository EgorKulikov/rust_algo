//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/algorithm/maximum-or-2-2a568ab6/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6 2\n1 3 7 0 6 1\n1 1000000000\n0\n","output":"15\n1000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let mut k: u64 = input.read();
    let mut a: Vec<u64> = input.read_vec(n);

    let mut ans = 0u64;
    for i in (0usize..64).rev() {
        let mut req = std::u64::MAX;
        let mut at = None;
        let all_bits = u64::all_bits(i);
        for (j, a) in a.iter().cloned().enumerate() {
            if a.is_set(i) {
                req = 0;
                at = Some(j);
            } else {
                let tail = a & all_bits;
                let delta = (1u64 << i) - tail;
                if delta < req {
                    req = delta;
                    at = Some(j);
                }
            }
        }
        if req <= k {
            ans.set_bit(i);
            k -= req;
            a[at.unwrap()] += req;
        }
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
