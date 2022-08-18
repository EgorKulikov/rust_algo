//{"name":"A2. Бурёнка и традиции (сложная версия)","group":"Codeforces - Codeforces Round #814 (Div. 1)","url":"https://codeforces.com/contest/1718/problem/A2","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n4\n5 5 5 5\n3\n1 3 2\n2\n0 0\n3\n2 5 7\n6\n1 2 3 3 2 1\n10\n27 27 34 32 2 31 23 56 52 4\n5\n1822 1799 57 23 55\n","output":"2\n2\n0\n2\n4\n7\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A2BuryonkaITraditsiiSlozhnayaVersiya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_unsigned_vec(n);

    let mut ans = n;
    let mut cur = 0;
    let mut was = HashSet::new();
    was.insert(0);
    for i in a {
        cur ^= i;
        if was.contains(&cur) {
            ans -= 1;
            cur = 0;
            was.clear();
        }
        was.insert(cur);
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
