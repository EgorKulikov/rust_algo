//{"name":"D. Нечётные запросы","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5 5\n2 2 1 3 2\n2 3 3\n2 3 4\n1 5 5\n1 4 9\n2 4 3\n10 5\n1 1 1 1 1 1 1 1 1 1\n3 8 13\n2 5 10\n3 8 10\n1 10 2\n1 9 100\n","output":"YES\nYES\nYES\nNO\nYES\nNO\nNO\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNechyotnieZaprosi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);

    let s = a.as_slice().partial_sums();
    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let k = input.read_long();
        let res = s[l] + s[n] - s[r] + (r - l).into_i64() * k;
        out_line!(res % 2 == 1);
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
}
//END MAIN
