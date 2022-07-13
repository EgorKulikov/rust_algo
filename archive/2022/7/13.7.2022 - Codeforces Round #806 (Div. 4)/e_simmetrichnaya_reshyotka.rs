//{"name":"E. Симметричная решётка","group":"Codeforces - Codeforces Round #806 (Div. 4)","url":"https://codeforces.com/contest/1703/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n010\n110\n010\n1\n0\n5\n11100\n11011\n01011\n10011\n11000\n5\n01000\n10101\n01010\n00010\n01001\n5\n11001\n00000\n11111\n10110\n01111\n","output":"1\n0\n9\n7\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESimmetrichnayaReshyotka"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_table::<char>(n, n);

    let mut ans = 0;
    for i in 0..(n + 1) / 2 {
        for j in 0..n / 2 {
            let mut r = i;
            let mut c = j;
            let mut ones = 0;
            for _ in 0..4 {
                if a[(r, c)] == '1' {
                    ones += 1;
                }
                let nr = c;
                let nc = n - 1 - r;
                r = nr;
                c = nc;
            }
            ans += ones.min(4 - ones);
        }
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
