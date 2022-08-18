//{"name":"A. Три двери","group":"Codeforces - Educational Codeforces Round 132 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1709/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 1 2\n1\n0 3 2\n2\n3 1 0\n2\n1 3 0\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATriDveri"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut x = input.read_usize();
    let behind = input.read_usize_vec(3);

    let mut has = 0;
    while x != 0 {
        has.set_bit(x - 1);
        x = behind[x - 1];
    }
    out_line!(has == 7);
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
