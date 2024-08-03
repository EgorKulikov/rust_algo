//{"name":"A. Сильный пароль","group":"Codeforces - Educational Codeforces Round 168 (Rated for Div. 2)","url":"https://codeforces.com/contest/1997/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\na\naaa\nabb\npassword\n","output":"wa\naada\nabcb\npastsword\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASilniiParol"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::{Str, StrReader};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();

    for i in 1..s.len() {
        if s[i - 1] == s[i] {
            out.print(Str::from(&s[..i]));
            if s[i] == b'a' {
                out.print('b');
            } else {
                out.print('a');
            }
            out.print_line(Str::from(&s[i..]));
            return;
        }
    }
    if s[0] == b'a' {
        out.print('b');
    } else {
        out.print('a');
    }
    out.print_line(s);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
