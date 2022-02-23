//{"name":"D. Bee and Honey","group":"TLX - TOKI Regular Open Contest #26","url":"https://tlx.toki.id/contests/troc-26/problems/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3 1\n2\n4 1\n10 -1\n2\n5 2\n8 1\n","output":"3\n"},{"input":"3 2\n2\n4 1\n10 -1\n2\n5 2\n10 1\n","output":"1\n"},{"input":"7 4\n2\n1 1\n7 -1\n3\n1 1\n5 2\n8 1\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBeeAndHoney"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let k = input.read_int();
    let l = input.read_int();
    let n = input.read_usize();
    let mut bees = input.read_int_pair_vec(n);
    let m = input.read_usize();
    let mut flowers = input.read_int_pair_vec(m);

    flowers.sort();
    bees.sort();
    let mut until = vec![None; m];
    let mut last = None;
    let mut at = 0;
    for (u, &(p, _)) in until.iter_mut().zip(flowers.iter()) {
        while at < n && bees[at].0 <= p {
            if bees[at].1 == 1 {
                last.maxim(bees[at].0);
            }
            at += 1;
        }
        if let Some(last) = last {
            u.minim(p - last);
        }
    }
    bees.reverse();
    at = 0;
    last = None;
    for (u, &(p, _)) in until.iter_mut().zip(flowers.iter()).rev() {
        while at < n && bees[at].0 >= p {
            if bees[at].1 == -1 {
                last.minim(bees[at].0);
            }
            at += 1;
        }
        if let Some(last) = last {
            u.minim(last - p);
        }
    }
    let mut good = until
        .into_iter()
        .zip(flowers.into_iter())
        .filter_map(|(v, (_, e))| v.map(|v| (v, e)))
        .collect_vec();
    good.sort();
    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    if k - l > good.len().into_i32() {
        out_line!(-1);
        return;
    }
    at = 0;
    for i in l..k {
        while at < good.len() && good[at].0 <= i {
            heap.push(Reverse(good[at].1));
            at += 1;
        }
        match heap.pop() {
            None => {
                out_line!(-1);
                return;
            }
            Some(v) => {
                ans += v.0.into_i64();
            }
        }
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
