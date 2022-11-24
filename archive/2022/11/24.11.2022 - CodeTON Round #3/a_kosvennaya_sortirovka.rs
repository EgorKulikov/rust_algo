//{"name":"A. Косвенная сортировка","group":"Codeforces - CodeTON Round 3 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1750/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n3\n1 2 3\n3\n1 3 2\n7\n5 3 4 7 6 2 1\n7\n7 6 5 4 3 2 1\n5\n2 1 4 5 3\n5\n2 1 3 4 5\n7\n1 2 6 7 4 3 5\n","output":"Yes\nYes\nNo\nNo\nNo\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AKosvennayaSortirovka"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);

    set_bool_output(BoolOutput::YesNo);
    out_line!(a[0] == 1);
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
