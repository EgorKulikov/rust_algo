//{"name":"Remainder Twist","group":"HackerEarth - March Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-23/algorithm/remainder-twist-987a698c/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5 7\n5 3\n5 26\n","output":"2\n3\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"RemainderTwist"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();
    let r = input.read_long();

    let mut left = 0;
    let mut right = n;
    let f = |x: i64| {
        let mut ans = 0;
        for i in x + 1..=n {
            let full = n / i;
            ans += full * (i - x) - if x == 0 { 1 } else { 0 };
            let start = full * i + x;
            if start <= n {
                ans += n - start + 1;
            }
        }
        ans
    };
    if f(0) < r {
        out_line!(-1);
        return;
    }
    while left < right {
        let mid = (left + right + 1) >> 1;
        if f(mid) >= r {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    out_line!(left);
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
