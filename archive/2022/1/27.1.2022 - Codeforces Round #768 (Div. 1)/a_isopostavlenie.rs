//{"name":"A. И-сопоставление","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 0\n4 1\n4 2\n4 3\n","output":"0 3\n1 2\n0 2\n1 3\n0 1\n2 3\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AISopostavlenie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();

    if k == n - 1 {
        if n <= 4 {
            out_line!(-1);
        } else {
            out_line!(n - 1, n - 2);
            out_line!(1, 3);
            out_line!(0, n - 4);
            for i in 2..n / 2 {
                if i != 3 {
                    out_line!(i, n - 1 - i);
                }
            }
        }
    } else if k == 0 {
        for i in 0..n / 2 {
            out_line!(i, n - 1 - i);
        }
    } else {
        out_line!(k, n - 1);
        out_line!(n - 1 - k, 0);
        for i in 1..n / 2 {
            if i != k && i + k != n - 1 {
                out_line!(i, n - 1 - i);
            }
        }
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
