//{"name":"A. Зачарованный лес","group":"Codeforces - Codeforces Round #796 (Div. 1)","url":"https://codeforces.com/contest/1687/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 2\n5 6 1 2 3\n5 7\n5 6 1 2 3\n1 2\n999999\n5 70000\n1000000000 1000000000 1000000000 1000000000 1000000000\n","output":"12\n37\n1000000\n5000349985\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZacharovanniiLes"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long_vec(n);

    if k <= n {
        let s = a.as_slice().partial_sums();
        let mut ans = None;
        let add = (k * (k - 1) / 2).into_i64();
        for i in 0..=n - k {
            ans.maxim(s[i + k] - s[i] + add);
        }
        out_line!(ans);
        return;
    }
    let ans = a.into_iter().sum::<i64>() + (k * n - n * (n + 1) / 2).into_i64();
    out_line!(ans);
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
