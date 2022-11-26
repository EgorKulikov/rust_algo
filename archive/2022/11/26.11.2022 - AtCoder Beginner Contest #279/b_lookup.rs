//{"name":"B - LOOKUP","group":"AtCoder - TOYOTA SYSTEMS Programming Contest 2022(AtCoder Beginner Contest 279)","url":"https://atcoder.jp/contests/abc279/tasks/abc279_b","interactive":false,"timeLimit":2000,"tests":[{"input":"voltage\ntag\n","output":"Yes\n"},{"input":"atcoder\nace\n","output":"No\n"},{"input":"gorilla\ngorillagorillagorilla\n","output":"No\n"},{"input":"toyotasystems\ntoyotasystems\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BLOOKUP"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::out_line;
use algo_lib::string::string::Str;
use algo_lib::string::string_algorithms::StringAlgorithms;

fn solve(input: &mut Input) {
    let s: Str = input.read();
    let t: Str = input.read();

    set_bool_output(BoolOutput::YesNo);
    let z = (t.clone() + s).as_slice().z_algorithm();
    for i in t.len()..z.len() {
        if z[i] == t.len() {
            out_line!(true);
            return;
        }
    }
    out_line!(false);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
