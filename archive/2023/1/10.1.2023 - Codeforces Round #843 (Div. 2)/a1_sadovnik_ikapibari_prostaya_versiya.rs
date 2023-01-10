//{"name":"A1. Садовник и капибары (простая версия)","group":"Codeforces - Codeforces Round #843 (Div. 2)","url":"https://codeforces.com/contest/1775/problem/A1","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nbbba\naba\naaa\nabba\nabbb\n","output":"b bb a\na b a\na a a\nab b a\na bb b\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A1SadovnikIKapibariProstayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    for i in 1..s.len() - 1 {
        if s[i] == b'a' {
            out_line!(Str::from(&s[..i]), "a", Str::from(&s[i + 1..]));
            return;
        }
    }
    out_line!(
        Str::from(&s[..1]),
        Str::from(&s[1..s.len() - 1]),
        Str::from(&s[s.len() - 1..])
    );
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
