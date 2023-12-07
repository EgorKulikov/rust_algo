//{"name":"#5 - Mostovi","group":"DMOJ - COCI '23 Contest 1","url":"https://dmoj.ca/problem/coci23c1p5","interactive":false,"timeLimit":3000,"tests":[{"input":"4 5\n1 2\n2 3\n3 4\n4 1\n1 3\n","output":"1\n"},{"input":"6 7\n1 2\n2 4\n2 6\n3 5\n6 1\n4 3\n2 5\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Mostovi"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::from_biedges(n, &edges);
    let mut dsu = DSU::new(n);
    let mut ans = 0;
    let mut used = BitSet::new(n);
    for i in 0..n {
        dsu.clear();
        for &(j, k) in &edges {
            if j != i && k != i {
                dsu.join(j, k);
            }
        }
        if dsu.set_count() > 3 {
            ans += graph[i].len();
        } else if dsu.set_count() == 3 {
            for e in &graph[i] {
                if dsu.size(e.to()) != 1 {
                    ans += 1;
                }
            }
        } else {
            let mut cut_points = || -> HashSet<usize> {
                let mut timer = 0;
                used.fill(false);
                let mut tin = vec![0; n];
                let mut fup = vec![0; n];
                let mut ans = HashSet::new();
                let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
                    let mut children = 0;
                    used.set(vert);
                    tin[vert] = timer;
                    fup[vert] = timer;
                    timer += 1;
                    for e in &graph[vert] {
                        if e.to() == prev || e.to() == i {
                            continue;
                        }
                        let to = e.to();
                        if used[to] {
                            fup[vert].minim(tin[to]);
                        } else {
                            f.call(to, vert);
                            let cand = fup[to];
                            fup[vert].minim(cand);
                            if fup[to] >= tin[vert] && prev != n {
                                ans.insert(vert);
                            }
                            children += 1;
                        }
                    }
                    if prev == n && children > 1 {
                        ans.insert(vert);
                    }
                });
                dfs.call(if i == 0 { 1 } else { 0 }, n);
                ans
            };
            let cp = cut_points();
            for e in &graph[i] {
                if cp.contains(&e.to()) {
                    ans += 1;
                }
            }
        }
    }
    out.print_line(ans / 2);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
