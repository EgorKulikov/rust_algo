//{"name":"B. Максимальная сумма","group":"Codeforces - Educational Codeforces Round 148 (Rated for Div. 2)","url":"https://codeforces.com/contest/1832/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5 1\n2 5 1 10 6\n5 2\n2 5 1 10 6\n3 1\n1 2 3\n6 1\n15 22 12 10 13 11\n6 2\n15 22 12 10 13 11\n5 1\n999999996 999999999 999999997 999999998 999999995\n","output":"21\n11\n3\n62\n46\n3999999986\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMaksimalnayaSumma"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let k = input.read_size();
    let mut a = input.read_long_vec(n);

    a.sort();
    let s = a.partial_sums();
    let mut ans = None;
    for i in 0..=k {
        ans.maxim(s[n - k + i] - s[2 * i]);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
