//{"name":"P3 - Snowy Slopes","group":"DMOJ - An Animal Contest 4","url":"https://dmoj.ca/problem/aac4p3","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n1 2\n4 4\n7 1\n-1 1\n2 1\n4 6\n-2 2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3SnowySlopes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;
use std::collections::{HashMap, HashSet};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let points: Vec<(i64, i64)> = input.read_vec(n);
    let stp: Vec<(i64, i64)> = input.read_vec(m);

    let mut ans = 0;
    let mut was = HashSet::new();
    for (k, d) in stp {
        let key = Rational::new(k, d);
        if was.contains(&key) {
            continue;
        }
        was.insert(key);
        let mut map = HashMap::new();
        for (x, y) in points.iter().cloned() {
            let val = y * d - x * k;
            if map.contains_key(&val) {
                ans += *map.get(&val).unwrap();
                *map.get_mut(&val).unwrap() += 1;
            } else {
                map.insert(val, 1i64);
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
