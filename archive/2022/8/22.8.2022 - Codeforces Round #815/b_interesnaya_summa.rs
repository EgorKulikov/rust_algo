//{"name":"B. Интересная сумма","group":"Codeforces - Codeforces Round #815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n8\n1 2 2 3 1 5 6 1\n5\n1 2 3 100 200\n4\n3 3 3 3\n6\n7 8 3 1 1 8\n","output":"9\n297\n0\n14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BInteresnayaSumma"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let mut a = input.read_long_vec(n);

    a.sort();
    out_line!(a[n - 1] + a[n - 2] - a[0] - a[1]);
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
