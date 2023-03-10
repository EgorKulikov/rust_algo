//{"name":"Minimum Squared Sum","group":"CodeChef - INCL2023","url":"https://www.codechef.com/INCL2023/problems/ICL_MINSUM","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n5\n2 4 6 8 10\n","output":"10\n"},{"input":"1\n5\n3 -5 -3 3 4\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinimumSquaredSum"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);

    if n == 1 {
        out_line!(0);
        return;
    }
    a.sort();
    let sum_sq = a.iter().map(|x| x * x).sum::<i64>();
    let sum = a.iter().sum::<i64>();
    let mut s = 0;
    let mut t = sum;
    let mut ans = None;
    for (i, a) in a.into_iter().enumerate().take(n - 1) {
        s += a;
        t -= a;
        let a = (i + 1).into_i64();
        let x = s / a;
        let mut left = None;
        for x in x - 1..=x + 1 {
            left.minim(x * x * a - 2 * x * s);
        }
        let b = (n - i - 1).into_i64();
        let y = t / b;
        let mut right = None;
        for y in y - 1..=y + 1 {
            right.minim(y * y * b - 2 * y * t);
        }
        ans.minim(left.unwrap() + right.unwrap() + sum_sq);
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
