//{"name":"B. Точки на плоскости","group":"Codeforces - Educational Codeforces Round 145 (Rated for Div. 2)","url":"https://codeforces.com/contest/1809/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n3\n5\n975461057789971042\n","output":"0\n1\n2\n987654321\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTochkiNaPloskosti"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_long();

    let mut left = 1;
    let mut right = 1_000_000_000;
    while left < right {
        let mid = (left + right) / 2;
        if mid * mid >= n {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    out_line!(left - 1);
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
