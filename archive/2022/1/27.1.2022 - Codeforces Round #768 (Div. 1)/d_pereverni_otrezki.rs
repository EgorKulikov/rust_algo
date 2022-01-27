//{"name":"D. Переверни отрезки","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n6 2\n0 6 -2 1 -4 5\n1 2\n7 1\n1 -1 1 -1 1 -1 1\n2\n5 1\n-1000000000 -1000000000 -1000000000 -1000000000 -1000000000\n1\n","output":"18\n5\n5000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPereverniOtrezki"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::gcd;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let mut a = input.read_long_vec(n);
    let b = input.read_usize_vec(m);

    let b = b.into_iter().fold(0, |g, a| gcd(g, a));
    let mut ans = None;
    for _ in 0..2 {
        for i in a.iter_mut().take(b) {
            *i *= -1;
        }
        let mut cand = 0;
        for i in 0..b {
            let (sum, min, num_neg) =
                a.iter()
                    .skip(i)
                    .step_by(b)
                    .fold((0, None, 0), |(sum, mut min, num_neg), &a| {
                        min.minim(a.abs());
                        (sum + a.abs(), min, num_neg + if a < 0 { 1 } else { 0 })
                    });
            if num_neg % 2 == 0 {
                cand += sum;
            } else {
                cand += sum - 2 * min.unwrap();
            }
        }
        ans.maxim(cand);
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
