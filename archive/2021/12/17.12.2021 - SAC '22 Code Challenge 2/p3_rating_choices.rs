//{"name":"P3 - Rating Choices","group":"DMOJ - SAC '22 Code Challenge 2","url":"https://dmoj.ca/problem/sac22cc2p3","interactive":false,"timeLimit":1000,"tests":[{"input":"3 1500\n1 2 3 500\n2 4 5 700\n3 6 7 -500\n4 8 9 100\n5 10 11 50\n6 12 13 1000\n7 14 15 50\n","output":"2800\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P3RatingChoices"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let r: i64 = input.read();
    let nodes: Vec<(usize, usize, usize, i64)> = input.read_vec((1 << n) - 1);

    let mut ans = vec![-1i64; 1 << (n + 1)];
    let mut graph = Graph::new(1 << (n + 1));
    for (u, v, w, d) in nodes {
        graph.add_edge(u, WeightedEdge::new(v, d));
        graph.add_edge(u, WeightedEdge::new(w, 0));
    }
    let mut rec = RecursiveFunction::new(|f, vert| {
        if ans[vert] != -1 {
            ans[vert]
        } else {
            ans[vert] = 0;
            for e in graph[vert].iter() {
                ans[vert].maxim(e.weight() + f.call(e.to()));
            }
            ans[vert]
        }
    });
    out_line!(rec.call(1) + r);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
