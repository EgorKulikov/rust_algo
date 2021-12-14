//{"name":"B. Потерянная биграмма","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n7\nab bb ba aa ba\n7\nab ba aa ab ba\n3\naa\n5\nbb ab bb\n","output":"abbaaba\nabaabaa\nbaa\nbbabb\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPoteryannayaBigramma"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::string::string::Str;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let s: Vec<Str> = input.read_vec(n - 2);

    let mut ans = String::new();
    ans.push(s[0][0] as char);
    for i in 1..n - 2 {
        if s[i][0] != s[i - 1][1] {
            ans.push(s[i - 1][1] as char);
        }
        ans.push(s[i][0] as char);
    }
    ans.push(s[n - 3][1] as char);
    if ans.len() < n {
        ans.push('a');
    }
    out_line!(ans);
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
