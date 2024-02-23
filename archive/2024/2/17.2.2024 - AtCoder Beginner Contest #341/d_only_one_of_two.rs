//{"name":"D - Only one of two","group":"AtCoder - Toyota Programming Contest 2024#2（AtCoder Beginner Contest 341）","url":"https://atcoder.jp/contests/abc341/tasks/abc341_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2 3 5\n","output":"9\n"},{"input":"1 2 3\n","output":"5\n"},{"input":"100000000 99999999 10000000000\n","output":"500000002500000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DOnlyOneOfTwo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::gcd::lcm;
use std::cmp::Ordering;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_long();
    let m = input.read_long();
    let k = input.read_long();

    let l = lcm(n, m);
    let per_period = l / n - 1 + l / m - 1;
    let periods = (k - 1) / per_period;
    let base = periods * l;
    let mut next_n = base + n;
    let mut next_m = base + m;
    let mut ans = 0;
    for _ in periods * per_period..k {
        match next_n.cmp(&next_m) {
            Ordering::Less => {
                ans = next_n;
                next_n += n;
            }
            Ordering::Equal => {
                unreachable!();
            }
            Ordering::Greater => {
                ans = next_m;
                next_m += m;
            }
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    tester::stress_test();
}
//END MAIN
