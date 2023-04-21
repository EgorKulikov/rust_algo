//{"name":"Binary XOR","group":"CodeChef - PRAD2023","url":"https://www.codechef.com/PRAD2023/problems/BIN_XOR","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n11111\n7\n0001000\n","output":"5\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BinaryXOR"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::StrReader;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let s = input.read_str();

    let ones = s.iter().count_eq(&b'1');
    if ones % 2 == 1 {
        out_line!(n);
    } else if ones == 0 {
        out_line!(n);
    } else {
        let first = s.iter().position(|c| c == b'1').unwrap();
        let last = s.iter().rposition(|c| c == b'1').unwrap();
        out_line!(n - (first + 1).min(n - last));
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
