//{"name":"B. Odd Swap Sort","group":"Codeforces - Codeforces Round #771 (Div. 2)","url":"https://codeforces.com/contest/1638/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4\n1 6 31 14\n2\n4 2\n5\n2 9 6 7 10\n3\n6 6 6\n","output":"Yes\nNo\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOddSwapSort"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let odd = a.iter().cloned().filter(|&i| i % 2 == 1).collect_vec();
    let even = a.iter().cloned().filter(|&i| i % 2 == 0).collect_vec();
    fn is_sorted(v: &[u32]) -> bool {
        for (&a, &b) in v.consecutive_iter() {
            if a > b {
                return false;
            }
        }
        true
    }
    out_line!(is_sorted(&odd) && is_sorted(&even));
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
