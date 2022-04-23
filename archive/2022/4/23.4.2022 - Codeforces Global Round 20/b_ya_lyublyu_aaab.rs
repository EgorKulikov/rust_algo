//{"name":"B. Я люблю AAAB","group":"Codeforces - Codeforces Global Round 20","url":"https://codeforces.com/contest/1672/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\nAABAB\nABB\nAAAAAAAAB\nA\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BYaLyublyuAAAB"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut delta = 0;
    for c in s.iter() {
        if c == b'A' {
            delta += 1;
        } else {
            delta -= 1;
        }
        if delta < 0 {
            out_line!(false);
            return;
        }
    }
    out_line!(s[s.len() - 1] == b'B');
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
