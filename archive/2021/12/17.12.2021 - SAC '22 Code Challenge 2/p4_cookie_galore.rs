//{"name":"P4 - Cookie Galore","group":"DMOJ - SAC '22 Code Challenge 2","url":"https://dmoj.ca/problem/sac22cc2p4","interactive":false,"timeLimit":1000,"tests":[{"input":"6 4\nC..C\n.CCC\nCC.C\n..CC\nCC..\nC.C.\n","output":"3\n"},{"input":"5 5\n.....\n..CC.\n.....\n.....\n.....\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P4CookieGalore"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let map = input.read_table::<char>(n, m);

    let mut graph = Graph::new(n * m);
    for i in 0..n {
        for j in 0..m {
            for (r, c) in D4::iter(i, j, n, m) {
                graph.add_edge(
                    i * m + j,
                    WeightedEdge::new(r * m + c, if map[(r, c)] == 'C' { 1 } else { 0 }),
                );
            }
        }
    }
    let ans = graph.distance(0, n * m - 1).unwrap().0 + if map[(0, 0)] == 'C' { 1 } else { 0 };
    out_line!(ans);
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
