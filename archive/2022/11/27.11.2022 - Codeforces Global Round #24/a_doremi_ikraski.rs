//{"name":"A. Дореми и краски","group":"Codeforces - Codeforces Global Round 24","url":"https://codeforces.com/contest/1764/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5\n1 3 2 2 4\n5\n1 2 3 4 5\n4\n2 1 2 1\n3\n2 3 3\n2\n2 2\n1\n1\n9\n9 8 5 2 1 1 2 3 3\n","output":"2 4\n1 5\n1 4\n2 3\n1 2\n1 1\n3 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADoremiIKraski"}}}

use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let _a = input.read_size_vec(n).dec_by_one();

    out_line!(1, n);
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
