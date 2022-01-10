//{"name":"#4 - Planine","group":"DMOJ - COCI '20 Contest 5","url":"https://dmoj.ca/problem/coci20c5p4","interactive":false,"timeLimit":2000,"tests":[{"input":"9 6\n0 0\n1 2\n3 0\n6 3\n8 1\n9 2\n11 1\n12 4\n14 0\n","output":"1\n"},{"input":"9 5\n-5 2\n-4 3\n-2 1\n0 4\n2 2\n3 3\n4 1\n5 2\n6 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Planine"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::rational::Rational;
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let h = input.read_long();
    let pts: Vec<(i64, i64)> = input.read_vec(n);
    let mut last = None;
    let mut next: Option<Rational<i64>> = None;

    let mut nx = vec![0; n];
    let mut ny = vec![0; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in (0..n).rev() {
        let (x, y) = pts[i];
        while q.len() > 2 {
            let p = q[q.len() - 1];
            let pp = q[q.len() - 2];
            let (x1, y1) = pts[p];
            let (x2, y2) = pts[pp];
            if y * (x2 - x1) + y2 * (x1 - x) >= y1 * (x2 - x) {
                q.pop_back();
            } else {
                break;
            }
        }
        if let Some(j) = q.back() {
            nx[i] = pts[*j].0;
            ny[i] = pts[*j].1;
        }
        q.push_back(i);
    }
    q.clear();
    let mut px = vec![0; n];
    let mut py = vec![0; n];
    for i in 0..n {
        let (x, y) = pts[i];
        while q.len() > 2 {
            let p = q[q.len() - 1];
            let pp = q[q.len() - 2];
            let (x1, y1) = pts[p];
            let (x2, y2) = pts[pp];
            if y * (x1 - x2) + y2 * (x - x1) >= y1 * (x - x2) {
                q.pop_back();
            } else {
                break;
            }
        }
        if let Some(j) = q.back() {
            px[i] = pts[*j].0;
            py[i] = pts[*j].1;
        }
        q.push_back(i);
    }
    let mut ans = 0;
    for i in (2..n - 1).step_by(2) {
        let (x, y) = pts[i];
        if next.is_some() && next.unwrap() <= Rational::new(x, 1) {
            ans += 1;
            last = next;
            next = None;
        }
        let left = Rational::new(x, 1) - Rational::new((x - px[i]) * (h - y), py[i] - y);
        if last.is_some() && last.unwrap() >= left {
            continue;
        }
        let right = Rational::new(x, 1) + Rational::new((nx[i] - x) * (h - y), ny[i] - y);
        next.minim(right);
    }
    if next.is_some() {
        ans += 1;
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
