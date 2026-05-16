//{"name":"q2","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"q2"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;
use std::collections::VecDeque;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1);

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut ans = vec![n; n];
    let mut q = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
        q[vert] = 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            q[vert] += f.call(e.to(), vert);
        }
        q[vert]
    });
    dfs.call(0, n);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> VecDeque<usize> {
        if q[vert] == 1 {
            ans[0].minim(if vert == 0 { 0 } else { 1 });
            return vec![0].into();
        }
        let mut best = 0;
        let mut best_at = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            if best.maxim(q[e.to()]) {
                best_at = e.to();
            }
        }
        let mut res = f.call(best_at, vert);
        res.push_front(1);
        while res.len() < q[vert] {
            res.push_back(n);
        }
        for e in &graph[vert] {
            if e.to() == prev || e.to() == best_at {
                continue;
            }
            let call = f.call(e.to(), vert);
            for i in (0..res.len()).rev() {
                for j in 0..call.len() {
                    if i + j + 1 >= res.len() {
                        break;
                    }
                    let cand = res[i] + call[j];
                    res[i + j + 1].minim(cand);
                }
                res[i] += 1;
            }
        }
        for i in 0..res.len() {
            ans[i].minim(res[i] + if vert == 0 { 0 } else { 1 });
        }
        res
    });
    dfs.call(0, n);
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
