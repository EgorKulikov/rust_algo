//{"name":"A. Построение строки","group":"Codeforces - Educational Codeforces Round 127 (Rated for Div. 2)","url":"https://codeforces.com/contest/1671/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"8\naaaabbb\nbbaaaaabbb\naaaaaa\nabab\na\nb\naaaab\nbbaaa\n","output":"YES\nYES\nYES\nNO\nNO\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APostroenieStroki"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let mut changes = vec![0];
    for (i, (&c, &d)) in s.as_slice().consecutive_iter().enumerate() {
        if c != d {
            changes.push(i + 1);
        }
    }
    changes.push(s.len());
    for (&i, &j) in changes.consecutive_iter() {
        if i + 1 == j {
            out_line!("NO");
            return;
        }
    }
    out_line!("YES");
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
