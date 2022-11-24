//{"name":"A. По середине","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n5 2 6\n14 3 4\n20 2 1\n1 2 3\n11 19 12\n10 8 20\n6 20 3\n4 1 3\n19 8 4\n","output":"5\n4\n2\n2\n12\n10\n6\n3\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APoSeredine"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut num = input.read_int_vec(3);

    num.sort();
    out_line!(num[1]);
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
