//{"name":"F - Endless Walk","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_f","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1 2\n2 3\n3 4\n4 2\n4 5\n","output":"4\n"},{"input":"3 2\n1 2\n2 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEndlessWalk"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::{IncDec, Qty};
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponents;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_usize_pair_vec(m).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, Edge::new(v));
    }
    let (color, condensed_graph) = graph.strongly_connected_components();
    let qty = color.qty();
    let mut ans = 0;
    let mut good = BitSet::new(qty.len());
    for (i, q) in qty.into_iter().enumerate().rev() {
        if q != 1 || condensed_graph[i].iter().find(|e| good[e.to()]).is_some() {
            good.set(i, true);
            ans += q;
        }
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
