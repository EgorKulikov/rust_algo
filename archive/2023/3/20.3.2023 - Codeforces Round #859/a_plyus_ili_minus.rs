//{"name":"A. Плюс или минус","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/0?e4f5ae4b42fbfda61290dfc0030ba520ac0d241bd87d06661d3e949667618ce2=1","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n1 2 3\n3 2 1\n2 9 -7\n3 4 7\n1 1 2\n1 1 0\n3 3 6\n9 9 18\n9 9 0\n1 9 -8\n1 9 10\n","output":"+\n-\n-\n+\n+\n-\n+\n+\n-\n-\n+\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APlyusIliMinus"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.read_int();
    let b = input.read_int();
    let c = input.read_int();

    out_line!(if a + b == c { "+" } else { "-" });
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
