//{"name":"A. Запрещённая подпоследовательность","group":"Codeforces - Codeforces Round #761 (Div. 2)","url":"https://codeforces.com/contest/1617/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\nabacaba\nabc\ncccba\nacb\ndbsic\nbac\nabracadabra\nabc\ndddddddddddd\ncba\nbbc\nabc\nac\nabc\n","output":"aaaacbb\nabccc\nbcdis\naaaaacbbdrr\ndddddddddddd\nbbc\nac\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AZapreshchyonnayaPodposledovatelnost"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut s: Str = input.read();
    let t: Str = input.read();

    s.sort();
    if t == "abc".into() && s[0] == b'a' {
        let mut ss = s.into_string();
        while ss.contains("bc") {
            ss = ss.replace("bc", "cb");
        }
        s = ss.into();
    }
    out_line!(s);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
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
