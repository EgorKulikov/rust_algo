//{"name":"B. Зеркало в строке","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"https://codeforces.com/contest/1616/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n10\ncodeforces\n9\ncbacbacba\n3\naaa\n4\nbbaa\n","output":"cc\ncbaabc\naa\nbb\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZerkaloVStroke"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let s: Str = input.read();

    if n == 1 || s[0] <= s[1] {
        let mut s: Str = s[0..1].into();
        s += s.clone();
        out_line!(s);
        return;
    }
    for i in 1..=n {
        if i == n || s[i] > s[i - 1] {
            let mut s: Str = s[0..i].into();
            let mut t = s.clone();
            t.reverse();
            s += t;
            out_line!(s);
            return;
        }
    }
    unreachable!();
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
