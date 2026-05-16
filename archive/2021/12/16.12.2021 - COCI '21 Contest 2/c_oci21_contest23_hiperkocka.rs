//{"name":"COCI '21 Contest 2 #3 Hiperkocka","group":"DMOJ","url":"https://dmoj.ca/problem/coci21c2p3","interactive":false,"timeLimit":1000,"tests":[{"input":"1\n0 1\n","output":"1\n0 1\n"},{"input":"2\n0 1\n1 2\n","output":"2\n0 1 3\n0 2 3\n"},{"input":"3\n0 1\n0 2\n0 3\n","output":"4\n0 1 2 4\n3 1 2 7\n5 1 4 7\n6 2 4 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COCI21Contest23Hiperkocka"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
use algo_lib::graph::dfs_order::DFSOrder;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let edges = input.read_vec::<(usize, usize)>(n);

    let mut graph = Graph::new(n + 1);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let order = graph.dfs_order().0;
    let mut corresponding = vec![vec![0; n + 1]; 1 << (n - 1)];
    for i in 0..corresponding.len() {
        if (2 * i).count_ones() % 2 == 0 {
            corresponding[i][0] = 2 * i;
        } else {
            corresponding[i][0] = 2 * i + 1;
        }
    }
    for i in 0..n {
        let edge = 1 << i;
        let from = order.iter().find(&(i + 1)).unwrap();
        let mut to = from;
        for e in graph[from].iter() {
            if order[e.to()] <= i {
                to = e.to();
                break;
            }
        }
        assert_ne!(to, from);
        for c in corresponding.iter_mut() {
            c[from] = c[to] ^ edge;
        }
    }
    out_line!(corresponding.len());
    output().print_per_line(&corresponding);
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
