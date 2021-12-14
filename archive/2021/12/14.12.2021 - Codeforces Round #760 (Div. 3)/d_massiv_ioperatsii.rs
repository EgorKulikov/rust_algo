//{"name":"D. Массив и операции","group":"Codeforces - Codeforces Round #760 (Div. 3)","url":"https://codeforces.com/contest/1618/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n7 3\n1 1 1 2 1 3 1\n5 1\n5 5 5 5 5\n4 2\n1 3 3 7\n2 0\n4 2\n9 2\n1 10 10 1 10 2 7 10 3\n","output":"2\n16\n0\n6\n16\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMassivIOperatsii"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let k: usize = input.read();
    let mut a: Vec<u32> = input.read_vec(n);

    a.sort_unstable();
    let mut ans = a.iter().take(n - 2 * k).cloned().sum::<u32>();
    let mut q = HashMap::new();
    for i in a.into_iter().skip(n - 2 * k) {
        if !q.contains_key(&i) {
            q.insert(i, 0usize);
        }
        *q.get_mut(&i).unwrap() += 1;
    }
    if k != 0 {
        let has_max = q.into_values().max().unwrap();
        if has_max > k {
            ans += (has_max - k) as u32;
        }
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
