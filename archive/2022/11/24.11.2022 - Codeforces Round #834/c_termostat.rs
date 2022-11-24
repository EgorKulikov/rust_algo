//{"name":"C. Термостат","group":"Codeforces - Codeforces Round  #834 (Div. 3)","url":"https://codeforces.com/contest/1759/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n3 5 6\n3 3\n0 15 5\n4 5\n0 10 5\n3 7\n3 5 6\n3 4\n-10 10 11\n-5 6\n-3 3 4\n1 0\n-5 10 8\n9 2\n1 5 1\n2 5\n-1 4 3\n0 2\n-6 3 6\n-1 -4\n","output":"0\n2\n3\n-1\n1\n-1\n3\n1\n3\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTermostat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let l = input.read_int();
    let r = input.read_int();
    let x = input.read_int();
    let a = input.read_int();
    let b = input.read_int();

    if a == b {
        out_line!(0);
    } else if (a - b).abs() >= x {
        out_line!(1);
    } else if a - l >= x && b - l >= x || r - a >= x && r - b >= x {
        out_line!(2);
    } else if a - l >= x && r - b >= x || r - a >= x && b - l >= x {
        out_line!(3);
    } else {
        out_line!(-1);
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
