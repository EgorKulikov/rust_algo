//{"name":"A. Да-да?","group":"Codeforces - Codeforces Round  #834 (Div. 3)","url":"https://codeforces.com/contest/1759/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"12\nYES\nesYes\ncodeforces\nes\nse\nYesY\nesYesYesYesYesYesYe\nseY\nYess\nsY\no\nYes\n","output":"NO\nYES\nNO\nYES\nNO\nYES\nYES\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADaDa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use algo_lib::string::string::Str;
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let s: Str = input.read();

    let map = [(b'Y', b'e'), (b'e', b's'), (b's', b'Y')]
        .into_iter()
        .collect::<HashMap<_, _>>();
    let mut cur = s[0];
    for c in s.iter().skip(1) {
        if let Some(&next) = map.get(&cur) {
            if c != next {
                out_line!(false);
                return;
            }
        } else {
            out_line!(false);
            return;
        }
        cur = c;
    }
    out_line!(map.contains_key(&cur));
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
