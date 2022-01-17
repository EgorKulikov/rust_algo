//{"name":"B. Небольшое сжатие","group":"Codeforces - Educational Codeforces Round 121 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1626/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n10057\n90\n","output":"10012\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNebolshoeSzhatie"}}}

use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let x: Str = input.read();

    for (i, (&a, &b)) in x.as_slice().consecutive_iter().enumerate().rev() {
        let mid = a - b'0' + b - b'0';
        if mid >= 10 {
            out_line!(format!(
                "{}{mid}{}",
                Str::from(&x[..i]),
                Str::from(&x[i + 2..])
            ));
            return;
        }
    }
    out_line!(format!(
        "{}{}",
        x[0] - b'0' + x[1] - b'0',
        Str::from(&x[2..])
    ));
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
