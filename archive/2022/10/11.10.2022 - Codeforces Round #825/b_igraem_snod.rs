//{"name":"B. Играем с НОД","group":"Codeforces - Codeforces Round #825 (Div. 2)","url":"https://codeforces.com/contest/1736/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n343\n2\n4 2\n3\n4 2 4\n4\n1 1 1 1\n","output":"YES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIgraemSNOD"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::gcd::{gcd, lcm};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_int_vec(n);

    let mut b = Vec::with_capacity(n + 1);
    b.push(a[0]);
    for (&x, &y) in a.consecutive_iter() {
        b.push(lcm(x, y));
    }
    b.push(a[n - 1]);
    for ((&x, &y), a) in b.consecutive_iter().zip(a.into_iter()) {
        if a != gcd(x, y) {
            out_line!(false);
            return;
        }
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
