//{"name":"C. Мастер игры","group":"Codeforces - Codeforces Round #758 (Div.1 + Div. 2)","url":"https://codeforces.com/contest/1608/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1 2 3 4\n1 2 3 4\n4\n11 12 20 21\n44 22 11 30\n1\n1000000000\n1000000000\n","output":"0001\n1111\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMasterIgri"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::io::Write;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let a: Vec<u32> = input.read_vec(n);
    let b: Vec<u32> = input.read_vec(n);

    let mut o1 = (0..n).collect_vec();
    o1.sort_by(|i, j| a[*j].cmp(&a[*i]));
    let mut o2 = (0..n).collect_vec();
    o2.sort_by(|i, j| b[*j].cmp(&b[*i]));
    let mut i = 1;
    let mut j = 0;
    let mut ans = 1;
    let mut min_a = a[o1[0]];
    let mut min_b = b[o1[0]];
    let mut was = vec![b'0'; n];
    was[o1[0]] = b'1';
    loop {
        while i < n && was[o1[i]] == b'1' {
            i += 1;
        }
        if i < n && a[o1[i]] > min_a {
            ans += 1;
            min_b.minim(b[o1[i]]);
            was[o1[i]] = b'1';
            continue;
        }
        while j < n && was[o2[j]] == b'1' {
            j += 1;
        }
        if j < n && b[o2[j]] > min_b {
            ans += 1;
            min_a.minim(a[o2[j]]);
            was[o2[j]] = b'1';
            continue;
        }
        break;
    }
    output().write(was.as_slice());
    out_line!();
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
