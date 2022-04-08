//{"name":"A grid","group":"HackerEarth - March Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-22/algorithm/grid-4-b8980c85/","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 3\n011\n111\n2 3\n111\n001\n1 5\n00000\n","output":"6\n5\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGrid"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_table::<char>(n, m);

    let mut ways = Arr2d::new(n + 1, m + 1, 0i64);
    let mut ans = 0;
    for i in (0..n).rev() {
        let mut to_right = 0;
        for j in (0..m).rev() {
            if a[(i, j)] == '1' {
                to_right += 1;
            } else {
                to_right = 0;
            }
            ways[(i, j)] = to_right.min(ways[(i + 1, j + 1)] + 1);
            ans += ways[(i, j)];
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
