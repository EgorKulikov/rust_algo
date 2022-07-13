//{"name":"G. Хороший ключ, плохой ключ","group":"Codeforces - Codeforces Round #806 (Div. 4)","url":"https://codeforces.com/contest/1703/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n4 5\n10 10 3 1\n1 2\n1\n3 12\n10 10 29\n12 51\n5 74 89 45 18 69 67 67 11 96 23 59\n2 57\n85 60\n","output":"11\n0\n13\n60\n58\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GKhoroshiiKlyuchPlokhoiKlyuch"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_long();
    let mut a = input.read_long_vec(n);

    let s = a.as_slice().partial_sums();
    let mut ans = s[n] - k * n.into_i64();
    for i in (0..n).rev() {
        let mut cur = s[i] - i.into_i64() * k;
        for j in i..(i + 30).min(n) {
            a[j] /= 2;
            cur += a[j];
        }
        ans.maxim(cur);
    }
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
