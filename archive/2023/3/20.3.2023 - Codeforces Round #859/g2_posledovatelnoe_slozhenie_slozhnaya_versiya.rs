//{"name":"G2. Последовательное сложение (сложная версия)","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/G2","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n1\n1\n2\n5\n5 1 3 2 1\n5\n7 1 5 2 1\n3\n1 1 1\n5\n1 1 4 2 1\n","output":"YES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G2PosledovatelnoeSlozhenieSlozhnayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut c = input.read_long_vec(n);

    c.sort();
    if c[0] != 1 {
        out_line!(false);
        return;
    }
    let mut sum = 1;
    for x in c.into_iter().skip(1) {
        if x > sum {
            out_line!(false);
            return;
        }
        sum += x;
    }
    out_line!(true);
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
