//{"name":"G - Distance Queries on a Tree","group":"AtCoder - AtCoder Beginner Contest 294","url":"https://atcoder.jp/contests/abc294/tasks/abc294_g","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n1 2 3\n1 3 6\n1 4 9\n4 5 10\n4\n2 2 3\n2 1 5\n1 3 1\n2 1 5\n","output":"9\n19\n11\n"},{"input":"7\n1 2 1000000000\n2 3 1000000000\n3 4 1000000000\n4 5 1000000000\n5 6 1000000000\n6 7 1000000000\n3\n2 1 6\n1 1 294967296\n2 1 6\n","output":"5000000000\n4294967296\n"},{"input":"1\n1\n2 1 1\n","output":"0\n"},{"input":"8\n1 2 105\n1 3 103\n2 4 105\n2 5 100\n5 6 101\n3 7 106\n3 8 100\n18\n2 2 8\n2 3 6\n1 4 108\n2 3 4\n2 3 5\n2 5 5\n2 3 1\n2 4 3\n1 1 107\n2 3 1\n2 7 6\n2 3 8\n2 1 5\n2 7 6\n2 4 7\n2 1 7\n2 5 3\n2 8 6\n","output":"308\n409\n313\n316\n0\n103\n313\n103\n525\n100\n215\n525\n421\n209\n318\n519\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GDistanceQueriesOnATree"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::graph::dfs_order::DFSOrder;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_size();
    let mut edges = input.read_vec::<(usize, usize, i64)>(n - 1);
    let q = input.read_size();

    let mut graph = Graph::new(n);
    for &(u, v, w) in &edges {
        graph.add_edge(u - 1, BiWeightedEdge::new(v - 1, w));
    }
    struct Node(i64);
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Node(0)
        }

        fn join(
            &mut self,
            _left_val: &Self,
            _right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
        }

        fn accumulate(&mut self, value: &Self, _left: usize, _right: usize) {
            self.0 += value.0;
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {
            self.0 = 0;
        }
    }
    let mut tree = SegmentTree::<Node>::new(n);
    let order = graph.dfs_order();
    let lca = graph.lca();
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
            tree.update(order.0[e.to()]..order.1[e.to()], &Node(e.weight()));
        }
    });
    dfs.call(0, n);
    for _ in 0..q {
        let t = input.read_size();
        match t {
            1 => {
                let at = input.read_size() - 1;
                let w = input.read_long();
                let delta = w - edges[at].2;
                edges[at].2 = w;
                let v = if lca.level(edges[at].0 - 1) < lca.level(edges[at].1 - 1) {
                    edges[at].1
                } else {
                    edges[at].0
                } - 1;
                tree.update(order.0[v]..order.1[v], &Node(delta));
            }
            2 => {
                let u = input.read_size() - 1;
                let v = input.read_size() - 1;
                let l = lca.lca(u, v);
                let ans = tree.point_query(order.0[u]).0 + tree.point_query(order.0[v]).0
                    - 2 * tree.point_query(order.0[l]).0;
                out_line!(ans);
            }
            _ => unreachable!(),
        }
    }
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
