//{"name":"D - Count Interval","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_d","interactive":false,"timeLimit":2000,"tests":[{"input":"6 5\n8 -3 5 7 0 -4\n","output":"3\n"},{"input":"2 -1000000000000000\n1000000000 -1000000000\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCountInterval"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashMap;

fn solve(input: &mut Input) {
    let n = input.read();
    let k: i64 = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let mut ans = 0u64;
    let mut map = HashMap::new();
    map.insert(0i64, 1u64);
    let mut sum = 0i64;
    for i in a {
        sum += i;
        if let Some(val) = map.get(&(sum - k)) {
            ans += *val;
        }
        if map.contains_key(&sum) {
            *map.get_mut(&sum).unwrap() += 1;
        } else {
            map.insert(sum, 1);
        }
    }
    out_line!(ans);
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
