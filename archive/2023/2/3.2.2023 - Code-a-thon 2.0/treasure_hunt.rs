//{"name":"Treasure Hunt","group":"CodeChef - CDAT2023","url":"https://www.codechef.com/CDAT2023/problems/T_HUNT","interactive":false,"timeLimit":3000,"tests":[{"input":"1\n5\n3 -4 5 1 -2\n-4 5\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TreasureHunt"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let d = input.read_long_vec(n);
    let lower = input.read_long();
    let upper = input.read_long();

    let mut min = 0;
    let mut max = 0;
    let mut cur = 0;
    for i in d {
        cur += i;
        min = min.min(cur);
        max = max.max(cur);
    }
    out_line!(((upper - lower) - (max - min) + 1).max(0));
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
