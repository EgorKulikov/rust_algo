//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut edges = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut dsu = DSU::new(n);
    let mut sets = Vec::with_capacity(n);
    for i in 0..n {
        sets.push(vec![i]);
    }
    edges.sort_by_key(|&(_, _, w)| w);
    let mut dist_from_start = vec![0; n];
    let mut dist_from_end = vec![0; n];
    for (mut u, mut v, w) in edges.iter().copied() {
        u = dsu.get(u);
        v = dsu.get(v);
        if u == v {
            continue;
        }
        if u == dsu.get(0) {
            for &i in &sets[v] {
                dist_from_start[i] = w;
            }
        }
        if v == dsu.get(0) {
            for &i in &sets[u] {
                dist_from_start[i] = w;
            }
        }
        if u == dsu.get(n - 1) {
            for &i in &sets[v] {
                dist_from_end[i] = w;
            }
        }
        if v == dsu.get(n - 1) {
            for &i in &sets[u] {
                dist_from_end[i] = w;
            }
        }
        if sets[u].len() < sets[v].len() {
            swap(&mut u, &mut v);
        }
        let mut vec = Vec::new();
        swap(&mut vec, &mut sets[v]);
        for i in vec {
            sets[u].push(i);
        }
        dsu.join(u, v);
        assert_eq!(dsu.get(u), u);
    }
    let mut ans = None;
    for (u, v, w) in edges {
        if w >= dist_from_start[u] && w >= dist_from_end[v] {
            ans.minim(w + dist_from_start[u].max(dist_from_end[v]));
        }
        if w >= dist_from_start[v] && w >= dist_from_end[u] {
            ans.minim(w + dist_from_start[v].max(dist_from_end[u]));
        }
    }
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    stress_test::stress_test(run, tester::check);
}
//END MAIN
