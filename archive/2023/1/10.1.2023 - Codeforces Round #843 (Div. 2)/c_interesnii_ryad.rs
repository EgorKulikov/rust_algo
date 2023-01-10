//{"name":"C. Интересный ряд","group":"Codeforces - Codeforces Round #843 (Div. 2)","url":"https://codeforces.com/contest/1775/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10 8\n10 10\n10 42\n20 16\n1000000000000000000 0\n","output":"12\n10\n-1\n24\n1152921504606846976\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CInteresniiRyad"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut n = input.read_u64();
    let x = input.read_u64();

    let mut ans = n;
    for i in 0.. {
        if (ans & x) != x {
            out_line!(-1);
            return;
        }
        if ans == x {
            out_line!(n);
            return;
        }
        if n.is_set(i) {
            n += 1 << i;
        }
        ans &= n;
    }
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
