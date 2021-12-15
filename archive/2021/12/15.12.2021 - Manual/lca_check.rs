//{"name":"lca_check","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"lca.in","pattern":null},"output":{"type":"file","fileName":"lca.out","pattern":null},"languages":{"java":{"taskClass":"lca_check"}}}

use algo_lib::collections::id::Id;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let q = input.read();
    let queries: Vec<(String, usize, usize)> = input.read_vec(q);

    let mut id = Id::new();
    for (_, u, v) in queries.iter() {
        id.get(*u);
        id.get(*v);
    }
    let id_to_num = id.by_id();
    let mut graph = Graph::new(id.len());
    for (_, u, v) in queries.iter().filter(|(s, _, _)| s.as_str() == "ADD") {
        graph.add_edge(id.get(*u), BiEdge::new(id.get(*v)));
    }
    let lca = graph.lca();
    for (_, u, v) in queries.iter().filter(|(s, _, _)| s.as_str() == "GET") {
        out_line!(id_to_num[lca.lca(id.get(*u), id.get(*v))]);
    }
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
