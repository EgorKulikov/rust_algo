//{"name":"A. Превратить A в B","group":"Codeforces - Codeforces Round #825 (Div. 2)","url":"https://codeforces.com/contest/1736/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n1 0 1\n0 0 1\n4\n1 1 0 0\n0 1 1 1\n2\n1 1\n1 1\n4\n1 0 0 1\n0 1 1 0\n1\n0\n1\n","output":"1\n2\n0\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APrevratitAVB"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::sign::Unsigned;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize_vec(n);
    let b = input.read_usize_vec(n);

    let dif10 = a
        .iter()
        .zip(b.iter())
        .filter(|(&a, &b)| a == 1 && b == 0)
        .count();
    let dif01 = a
        .iter()
        .zip(b.iter())
        .filter(|(&a, &b)| a == 0 && b == 1)
        .count();
    if dif10.min(dif01) == 0 {
        out_line!(dif10 + dif01);
    } else {
        out_line!(dif10.distance(dif01) + 1);
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
