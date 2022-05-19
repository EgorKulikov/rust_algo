//{"name":"F. Разнообразные отрезки","group":"Codeforces - Codeforces Round #792 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1684/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n7 3\n1 1 2 1 3 3 5\n1 4\n4 5\n2 4\n5 2\n10 1 6 14 1\n4 5\n2 4\n4 5\n5 7 5 6\n2 2\n1 3\n2 4\n3 3\n3 4\n7 3\n2 2 2 7 8 2 2\n4 4\n4 4\n5 5\n1 1\n123\n1 1\n","output":"2\n0\n1\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRaznoobraznieOtrezki"}}}

extern crate core;

use algo_lib::collections::default_map::DefaultMap;
use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let a = input.read_usize_vec(n);
    let mut segs = input.read_usize_pair_vec(m);

    segs.sort();
    let mut n_segs = Vec::new();
    let mut right = 0;
    for (l, r) in segs {
        if r > right {
            n_segs.push((l - 1, r));
            right = r;
        }
    }
    segs = n_segs;

    let in_seg = |l: usize, r: usize| -> bool {
        let mut left = 0;
        let mut right = segs.len();
        while left < right {
            let mid = (left + right) / 2;
            if segs[mid].1 <= r {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left != segs.len() && l >= segs[left].0
    };

    let mut pos: DefaultMap<usize, Vec<usize>> = DefaultMap::new();
    for (i, a) in a.into_iter().enumerate() {
        pos[a].push(i);
    }
    let mut left = None;
    let mut right = None;
    let mut choose = Vec::new();
    for p in pos.into_values() {
        let mut l = None;
        let mut r = None;
        for (&i, &j) in p.consecutive_iter() {
            if in_seg(i, j) {
                if l.is_none() {
                    l = Some(i);
                }
                r = Some(j);
            }
        }
        if l.is_none() {
            continue;
        }
        let l = l.unwrap();
        let r = r.unwrap();
        let l_next = p[p.iter().find(&l).unwrap() + 1];
        let r_prev = p[p.iter().find(&r).unwrap() - 1];
        left.minim(l_next);
        right.maxim(r_prev);
        if in_seg(l, r) {
            choose.push((l, r));
        }
    }
    choose.sort();
    if left.is_none() {
        out_line!(0);
        return;
    }
    let left = left.unwrap();
    let mut right = right.unwrap();
    for &(i, _) in choose.iter() {
        right.maxim(i);
    }
    let mut ans = None;
    for (i, j) in choose {
        if i >= left {
            break;
        }
        ans.minim(right - i + 1);
        right.maxim(j);
    }
    ans.minim(right - left + 1);
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
