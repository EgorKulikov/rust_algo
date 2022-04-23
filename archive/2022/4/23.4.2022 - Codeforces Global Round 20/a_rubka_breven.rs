//{"name":"A. Рубка бревен","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n2 4 2 1\n1\n1\n","output":"errorgorn\nmaomao90\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARubkaBreven"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, BoolOutput};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    output().bool_output = BoolOutput::Custom("errorgorn", "maomao90");
    let s = a.into_iter().sum::<i32>() - n.into_i32();
    out_line!(s % 2 != 0);
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
