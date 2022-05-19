//{"name":"FIICode 2022 A1","group":"CS Academy - FIICode 2022 Round #1 – Powered by Atek Software","url":"https://csacademy.com/contest/fiicode-2022-round-1/task/fiicode-2022-a1/","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5 1 14\n5 1 15\n5 1 16\n4 4 25\n100 10000 10000000\n9876543210 123456789 1000000000000000000\n","output":"0\n1\n0\n1\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FIICode2022A1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n: u128 = input.read();
    let m: u128 = input.read();
    let s: u128 = input.read();

    let boxes = n * (n + 1) / 2;
    if boxes > s || boxes * m < s {
        out_line!(0);
    } else {
        out_line!(1);
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
}
//END MAIN
