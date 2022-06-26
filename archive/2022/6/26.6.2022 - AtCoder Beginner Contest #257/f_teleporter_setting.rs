//{"name":"F - Teleporter Setting","group":"AtCoder - NS Solutions Corporation Programming Contest 2022（AtCoder Beginner Contest 257）","url":"https://atcoder.jp/contests/abc257/tasks/abc257_f","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\n0 2\n1 2\n","output":"-1 -1 2\n"},{"input":"5 5\n1 2\n1 3\n3 4\n4 5\n0 2\n","output":"3 3 3 3 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTeleporterSetting"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges = input.read_usize_pair_vec(m);

    let mut graph = Graph::new(n + 1);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let d1 = graph.edge_distances(1);
    let dn = graph.edge_distances(n);
    let mut ans = Vec::with_capacity(n);
    for i in 1..=n {
        let mut res = d1[n];
        let left = d1[i].min(d1[0]);
        let right = dn[i].min(dn[0]);
        if left != std::u32::MAX && right != std::u32::MAX {
            res.minim(left + right);
        }
        if res != std::u32::MAX {
            ans.push(Some(res));
        } else {
            ans.push(None);
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
