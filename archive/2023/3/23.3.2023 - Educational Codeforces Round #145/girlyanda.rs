//{"name":"Гирлянда","group":"Codeforces","url":"https://m1.codeforces.com/contest/1809/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n9546\n0000\n3313\n","output":"4\n-1\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Girlyanda"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let mut s = input.read_vec::<char>(4);

    s.sort();
    if s[0] == s[3] {
        out_line!(-1);
        return;
    }
    if s[0] == s[2] || s[1] == s[3] {
        out_line!(6);
        return;
    }
    out_line!(4);
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
