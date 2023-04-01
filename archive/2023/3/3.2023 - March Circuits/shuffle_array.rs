//{"name":"Shuffle Array","group":"HackerEarth - March Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-23/approximate/shuffle-array-3df1e5bb/","interactive":false,"timeLimit":1000,"tests":[{"input":"4 2\n3 4 1 2\n","output":"2\n4\n2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ShuffleArray"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::random::random;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::cmp::Reverse;
use std::collections::{BTreeSet, VecDeque};
use std::time::{Duration, Instant};

static mut TOTAL_SCORE: usize = 0;

fn solve(input: &mut Input, _test_case: usize) {
    let start = Instant::now();
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);

    let mut edges = BTreeSet::new();
    edges.insert(n);
    edges.insert(0);
    let mut order = (0..n).collect_vec();
    order.sort_by_key(|&i| Reverse(a[i]));
    let mut special = Vec::new();
    for i in order {
        if edges.len() == k + 1
            || edges.len() == k && !edges.contains(&(i + 1)) && !edges.contains(&i)
        {
            break;
        }
        edges.insert(i);
        edges.insert(i + 1);
        special.push(i);
    }
    if edges.len() == k {
        for i in 1..n {
            if !edges.contains(&i) {
                edges.insert(i);
                break;
            }
        }
    }
    edges.remove(&0);
    special.reverse();
    let edges = edges.into_iter().collect_vec();
    // let mut edges = Vec::with_capacity(k);
    // for i in 1..=k {
    //     edges.push(i * n / k);
    // }
    let mut bs = BitSet::new(k);
    // let mut special = Vec::new();
    for i in &mut special {
        *i = edges.lower_bound(&(*i + 1));
        bs.set(*i, true);
    }
    let mut best_score = None;
    let mut best_ans = VecDeque::new();
    let mut best_lds = 0;
    let mut best_lis = 0;
    let r = random();
    fn lis_impl(a: &[i32]) -> usize {
        let mut dp = Vec::new();
        for &x in a {
            let i = dp.lower_bound(&x);
            if i == dp.len() {
                dp.push(x);
            } else {
                dp[i] = x;
            }
        }
        dp.len()
    }
    while Instant::now().duration_since(start) < Duration::from_millis(2950) {
        /*let mut ans = VecDeque::new();
        let mut principal = Vec::new();
        let mut per = (0..k).collect_vec();
        for i in 1..k {
            per.swap(i, r.next((i + 1).into_u64()).into_usize());
        }
        for i in per {
            if !bs[i] {
                ans.push_back(i + 1);
                let start = if i == 0 { 0 } else { edges[i - 1] };
                let end = edges[i];
                for j in start..end {
                    principal.push(a[j]);
                }
            }
        }*/
        let mut rep = Vec::new();
        for i in 0..k {
            if !bs[i] {
                let start = if i == 0 { 0 } else { edges[i - 1] };
                let end = edges[i];
                rep.push((
                    Reverse(a[start + r.next((end - start).into_u64()).into_usize()]),
                    i,
                ));
            }
        }
        rep.sort();
        let mut ans = VecDeque::new();
        let mut principal = VecDeque::new();
        for (i, (_, j)) in rep.into_iter().enumerate() {
            let start = if j == 0 { 0 } else { edges[j - 1] };
            let end = edges[j];
            if true {
                ans.push_back(j + 1);
                for j in start..end {
                    principal.push_back(a[j]);
                }
            } else {
                ans.push_front(j + 1);
                for j in (start..end).rev() {
                    principal.push_front(a[j]);
                }
            }
        }
        let mut principal = Vec::from(principal);
        let mut lis = lis_impl(&principal);
        principal.reverse();
        let mut lds = lis_impl(&principal);
        for &i in &special {
            if lis < lds {
                ans.push_back(i + 1);
                lis += 1;
            } else {
                ans.push_front(i + 1);
                lds += 1;
            }
        }
        let score = lis * lds - (lis ^ lds);
        if best_score.maxim(score) {
            best_ans = ans;
            best_lds = lds;
            best_lis = lis;
        }
    }
    output().print_per_line(&edges);
    let ans = Vec::from(best_ans);
    let lds = best_lds;
    let lis = best_lis;
    out_line!(ans);
    let exp_score = lis * lds - (lis ^ lds);
    eprintln!("Score = {}", exp_score);
    fn score(a: &[i32], edges: &[usize], p: &[usize]) -> usize {
        let mut ra = Vec::with_capacity(a.len());
        for &i in p {
            let start = if i == 1 { 0 } else { edges[i - 2] };
            let end = edges[i - 1];
            ra.extend_from_slice(&a[start..end]);
        }
        let lis = lis_impl(&ra);
        ra.reverse();
        let lds = lis_impl(&ra);
        lis * lds - (lis ^ lds)
    }
    let act_score = score(&a, &edges, &ans);
    //    assert_eq!(exp_score, act_score);
    unsafe {
        TOTAL_SCORE += exp_score;
    }
}

#[test]
fn generate() {
    use algo_lib::misc::random::random;
    use algo_lib::numbers::num_traits::primitive::Primitive;
    use std::fs::File;
    let n = 1000;
    let mut a = vec![0; n];
    let r = random();
    for i in 1..=50 {
        unsafe {
            algo_lib::io::output::OUTPUT = Some(algo_lib::io::output::Output::new(Box::new(
                File::create(format!("tests/{}.in", i)).unwrap(),
            )));
        }
        let k = (50 + r.next(51)).into_usize();
        out_line!(n, k);
        for j in 0..n {
            a[j] = r.next(100000) + 1;
        }
        out_line!(a);
        output().flush();
    }
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
    unsafe {
        eprintln!("Average score = {}", (TOTAL_SCORE as f64) / 50.);
    }
}
//END MAIN
