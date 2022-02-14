//{"name":"D. Yet Another Minimization Problem","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1\n3\n6\n4\n3 6 6 6\n2 7 4 1\n4\n6 7 2 4\n2 5 3 5\n","output":"0\n987\n914\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYetAnotherMinimizationProblem"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::mem::swap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_usize_vec(n);
    let mut b = input.read_usize_vec(n);

    for (i, j) in a.iter_mut().zip(b.iter_mut()) {
        if i > j {
            swap(i, j);
        }
    }
    let sum_a: usize = a.iter().sum();
    let sum_b: usize = b.iter().sum();
    let sum_sq = a.iter().chain(b.iter()).fold(0, |s, &i| s + i * i);
    if n == 1 {
        out_line!(0);
        return;
    }
    let mut reachable = BitSet::new(sum_b - sum_a + 1);
    reachable.set(0, true);
    let mut s = 0;
    for (&a, &b) in a.iter().zip(b.iter()) {
        let delta = b - a;
        s += delta;
        for i in (delta..=s).rev() {
            if reachable[i - delta] {
                reachable.set(i, true);
            }
        }
    }
    let mut target = (sum_b - sum_a) / 2;
    while !reachable[target] {
        target -= 1;
    }
    let up_sum = sum_a + target;
    let down_sum = sum_b - target;
    out_line!(up_sum * up_sum + down_sum * down_sum + (n - 2) * sum_sq);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
