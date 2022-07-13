//{"name":"D. Двойные строки","group":"Codeforces - Codeforces Round #806 (Div. 4)","url":"https://codeforces.com/contest/1703/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\nabab\nab\nabc\nabacb\nc\n3\nx\nxx\nxxx\n8\ncodeforc\nes\ncodes\ncod\nforc\nforces\ne\ncode\n","output":"10100\n011\n10100101\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDvoinieStroki"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Vec<Str> = input.read_vec(n);

    let set = s.iter().map(|it| it.to_string()).collect::<HashSet<_>>();
    let mut ans = Str::from(vec![b'0'; n]);
    for (s, a) in s.into_iter().zip(ans.iter_mut()) {
        for i in 1..s.len() {
            if set.contains(&Str::from(&s[..i]).to_string())
                && set.contains(&Str::from(&s[i..]).to_string())
            {
                *a = b'1';
                break;
            }
        }
    }
    out_line!(ans);
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
