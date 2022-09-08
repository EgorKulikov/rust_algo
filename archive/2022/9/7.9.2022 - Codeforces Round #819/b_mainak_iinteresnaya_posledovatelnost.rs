//{"name":"B. Mainak и интересная последовательность","group":"Codeforces - Codeforces Round #819 (Div. 1 + Div. 2) and Grimoire of Code Annual Contest 2022","url":"https://codeforces.com/contest/1726/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 3\n6 12\n2 1\n3 6\n","output":"Yes\n3\nYes\n1 3 2 2 3 1\nNo\nYes\n2 2 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMainakIInteresnayaPosledovatelnost"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();

    set_bool_output(BoolOutput::YesNo);
    if n > m || n % 2 == 0 && m % 2 == 1 {
        out_line!(false);
        return;
    }
    out_line!(true);
    let mut ans = Vec::with_capacity(n);
    for _ in 0..(n - 1) / 2 * 2 {
        ans.push(1);
    }
    if n % 2 == 1 {
        ans.push(m - n + 1);
    } else {
        ans.push((m - n + 2) / 2);
        ans.push((m - n + 2) / 2);
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
