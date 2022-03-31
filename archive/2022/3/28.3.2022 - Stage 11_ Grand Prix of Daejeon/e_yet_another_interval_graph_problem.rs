//{"name":"E. Yet Another Interval Graph Problem","group":"Yandex - Stage 11: Grand Prix of Daejeon","url":"https://official.contest.yandex.com/opencupXXII/contest/35265/problems/E/","interactive":false,"timeLimit":1000,"tests":[{"input":"5 2\n1 4 1\n3 6 2\n5 8 5\n7 10 2\n9 12 1\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EYetAnotherIntervalGraphProblem"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Detuple;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{compress, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let k = input.read_usize();
    let segments: Vec<(i32, i32, i64)> = input.read_vec(n);

    let (s, e, w) = segments.detuple();
    let (all, (s, e)) = compress!(s, e);
    let len = all.len();
    let mut ends = vec![Vec::new(); len];
    for (i, &e) in e.iter().enumerate() {
        ends[e].push(i);
    }
    let mut starts = vec![0; len];
    for (&s, &w) in s.iter().zip(w.iter()) {
        starts[s] += w;
    }
    let mut q = Arr2d::new(len, len, 0);
    let mut in_back = BitSet::new(n);
    let mut front = Vec::with_capacity(n);
    for i in 0..len {
        let mut cur = Vec::new();
        for (j, &s) in s.iter().enumerate() {
            if s >= i {
                cur.push((w[j], j));
            }
        }
        cur.sort();
        let mut cur_w = 0;
        for &(w, j) in cur.iter().take(cur.len().saturating_sub(k)) {
            front.push((w, j));
            cur_w += w;
        }
        for &(_, j) in cur.iter().skip(cur.len().saturating_sub(k)) {
            in_back.set(j, true);
        }
        for j in (i..len).rev() {
            q[(i, j)] = cur_w;
            cur_w -= starts[j];
            for &k in &ends[j] {
                if s[k] < i {
                    continue;
                }
                if in_back[k] {
                    in_back.set(k, false);
                    cur_w += w[k];
                    while let Some((w, l)) = front.pop() {
                        if e[l] >= j {
                            continue;
                        }
                        cur_w -= w;
                        in_back.set(l, true);
                        break;
                    }
                }
            }
        }
    }
    let mut ans = Vec::with_capacity(len + 1);
    ans.push(0);
    for i in 1..=len {
        let mut cur = q[(0, i - 1)];
        for j in 1..i {
            cur.minim(ans[j] + q[(j, i - 1)]);
        }
        ans.push(cur);
    }
    out_line!(ans[len]);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
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
