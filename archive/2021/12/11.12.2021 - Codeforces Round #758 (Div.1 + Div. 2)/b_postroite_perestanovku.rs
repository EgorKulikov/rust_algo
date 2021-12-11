//{"name":"B. Постройте перестановку","group":"Codeforces - Codeforces Round #758 (Div.1 + Div. 2)","url":"https://codeforces.com/contest/1608/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 1 1\n6 1 2\n6 4 0\n","output":"1 3 2 4\n4 2 3 1 5 6\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPostroitePerestanovku"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let a: usize = input.read();
    let b: usize = input.read();

    if a.max(b) - a.min(b) > 1 || a + b > n - 2 {
        out_line!("-1");
        return;
    }
    let mut ans = vec![0usize; n];
    let (min_start, max_start) = if a < b { (1, 2) } else { (2, 1) };
    let mut next_min = 1usize;
    for i in 0..b {
        ans[2 * i + min_start] = next_min;
        next_min += 1;
    }
    let mut next_max = n;
    for i in 0..a {
        ans[2 * i + max_start] = next_max;
        next_max -= 1;
    }
    if a <= b {
        for i in 0..n {
            if ans[i] == 0 {
                ans[i] = next_min;
                next_min += 1;
            }
        }
    } else {
        for i in (0..n).rev() {
            if ans[i] == 0 {
                ans[i] = next_min;
                next_min += 1;
            }
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
