//{"name":"J. Go Down The Tree","group":"Codeforces - STAR Contest 2022","url":"https://starcontest22.contest.codeforces.com/group/ZbfYu7B821/contest/378214/problem/J","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3 1 7 1 7 5\n","output":"4 6 7 7 7 7 7\n"},{"input":"4\n1 1 1\n","output":"2 3 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JGoDownTheTree"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let p = input.read_usize_vec(n - 1);

    let mut graph = Graph::new(n);
    for (i, t) in p.into_iter().enumerate() {
        graph.add_edge(t - 1, Edge::new(i + 1));
    }
    let mut res = vec![1; n];
    let mut next = vec![None; n];
    let mut dfs = RecursiveFunction::new(|f, vert: usize| {
        let mut best = None;
        for e in &graph[vert] {
            f.call(e.to());
            best.maxim((res[e.to()], e.to()));
        }
        if let Some((r, i)) = best {
            res[vert] = r + 1;
            next[vert] = Some(i);
        }
    });
    dfs.call(0);
    let mut heap = BinaryHeap::new();
    heap.push((res[0], 0));
    let mut cur = 0;
    let mut ans = Vec::with_capacity(n);
    while let Some((r, mut i)) = heap.pop() {
        cur += r;
        loop {
            for e in &graph[i] {
                if Some(e.to()) != next[i] {
                    heap.push((res[e.to()], e.to()));
                }
            }
            if let Some(next) = next[i] {
                i = next;
            } else {
                break;
            }
        }
        ans.push(cur);
    }
    assert_eq!(cur, n.into_i32());
    while ans.len() < n {
        ans.push(cur);
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
