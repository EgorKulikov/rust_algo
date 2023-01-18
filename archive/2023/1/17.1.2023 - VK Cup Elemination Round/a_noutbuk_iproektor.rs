//{"name":"A. Ноутбук и проектор","group":"Codeforces - VK Cup 2022 - Отборочный раунд (Engine)","url":"https://codeforces.com/contest/1781/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n55 20 29\n23 10 18 3\n20 10 5\n1 5 2 5\n15 15 4\n7 13 10 10\n2 1000 2\n1 1 1 999\n10 4 10\n7 1 2 1\n","output":"47\n8\n14\n1002\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANoutbukIProektor"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let w = input.read_int();
    let d = input.read_int();
    let h = input.read_int();
    let a = input.read_int();
    let b = input.read_int();
    let f = input.read_int();
    let g = input.read_int();

    let d1 = (a - f).abs();
    let d2 = (a + f).min(2 * w - a - f);
    let d3 = (b - g).abs();
    let d4 = (b + g).min(2 * d - b - g);
    out_line!(h + (d1 + d4).min(d2 + d3));
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
