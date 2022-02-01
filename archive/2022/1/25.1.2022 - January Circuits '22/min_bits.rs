//{"name":"Min Bits","group":"HackerEarth - January Circuits '22","url":"https://www.hackerearth.com/challenges/competitive/january-circuits-022/algorithm/min-bits-3-1c87d621/","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n10 1000\n","output":"16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MinBits"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::number_iterator::iterate_with_base;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let l = input.read_long();
    let r = input.read_long();

    let mut ans = None;
    for (_, _, number) in iterate_with_base(l, r, 2) {
        ans.minim((number.count_ones(), number));
    }
    out_line!(ans.unwrap().1);
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
