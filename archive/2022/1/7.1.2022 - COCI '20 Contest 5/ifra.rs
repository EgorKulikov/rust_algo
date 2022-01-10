//{"name":"#1 - Šifra","group":"DMOJ - COCI '20 Contest 5","url":"https://dmoj.ca/problem/coci20c5p1","interactive":false,"timeLimit":1000,"tests":[{"input":"abc123abc2a3a1\n","output":"4\n"},{"input":"borna123vitez\n","output":"1\n"},{"input":"as23dkrf23smk1asd23sam9\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Ifra"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::HashSet;

fn solve(input: &mut Input) {
    let s: Str = input.read();

    let mut set = HashSet::new();
    let mut cur = -1;
    for c in s {
        if c.is_ascii_digit() {
            if cur == -1 {
                cur = 0;
            }
            cur *= 10;
            cur += (c - b'0').into_i32();
        } else if cur != -1 {
            set.insert(cur);
            cur = -1;
        }
    }
    if cur != -1 {
        set.insert(cur);
    }
    out_line!(set.len());
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
