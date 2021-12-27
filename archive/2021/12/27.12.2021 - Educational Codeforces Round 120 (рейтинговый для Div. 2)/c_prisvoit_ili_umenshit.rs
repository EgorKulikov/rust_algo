//{"name":"C. Присвоить или уменьшить","group":"Codeforces - Educational Codeforces Round 120 (рейтинговый для Div. 2)","url":"http://codeforces.com/contest/1622/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 10\n20\n2 69\n6 9\n7 8\n1 2 1 3 1 2 1\n10 1\n1 2 3 1 2 6 1 6 8 10\n","output":"10\n0\n2\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPrisvoitIliUmenshit"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_long();
    let mut a = input.read_long_vec(n);

    let mut min = a.iter().cloned().min().unwrap();
    let mut add = 0;
    if min * n.into_i64() > k {
        let delta = min * n.into_i64() - k;
        let times = (delta + n.into_i64() - 1) / n.into_i64();
        min -= times;
        add = times;
    }
    a.sort_unstable();
    a[0] = min;
    let mut sum = a.iter().sum::<i64>();
    let mut ans = i64::MAX;
    for (i, v) in a.into_iter().rev().enumerate() {
        if sum <= k {
            ans.minim(add);
            out_line!(ans);
            return;
        }
        let cur_add = add + (sum - k + i.into_i64()) / (i + 1).into_i64();
        ans.minim(cur_add);
        add += 1;
        sum -= v - min;
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
