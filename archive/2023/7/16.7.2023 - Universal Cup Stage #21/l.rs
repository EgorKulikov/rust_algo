//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let y = input.read_long();
    let x = input.read_long();

    if x <= y {
        out_line!(y - x);
        return;
    }

    fn f(x: i64) -> i64 {
        let mut left = 1;
        let mut right = 2000000000;
        while left < right {
            let mid = (left + right) / 2;
            let val = mid * (mid + 1) / 2;
            if val >= x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    fn to_1(x: i64) -> i64 {
        let x1 = f(x);
        let x2 = x1 * (x1 + 1) / 2;
        x2 - x + x1 - 1
    }

    let x1 = f(x);
    let x2 = x1 * (x1 + 1) / 2;
    let x_to_1 = to_1(x);
    let delta = x2 - x;

    let y1 = f(y);
    let y2 = y1 * (y1 + 1) / 2;
    let y3 = y1 * (y1 - 1) / 2;
    if y2 - delta <= y {
        let cur = (y2 - delta).max(y3 + 1);
        out_line!(x_to_1 - to_1(cur) + y - cur);
        return;
    }
    let y4 = (y1 - 1) * (y1 - 2) / 2;
    let cur = (y3 - delta).max(y4 + 1);
    out_line!(x_to_1 - to_1(cur) + y - cur);
}

#[test]
fn test() {
    fn f(x: i64) -> i64 {
        x + (((2 * x) as f64).sqrt().round() as i64) + 1
    }

    for mut y in 1..=100 {
        let mut ans = 0;
        while y > 1 {
            print!("{y} ");
            let mut left = 2 * y - f(y);
            let mut val = f(left);
            while val < y {
                left += 1;
                val = f(left);
            }
            ans += 1 + val - y;
            y = left;
        }
        println!("1 ans = {ans}");
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
