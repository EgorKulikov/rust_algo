//{"name":"G - SCC","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_g","interactive":false,"timeLimit":5000,"tests":[{"input":"6 7\n1 4\n5 2\n3 0\n5 5\n4 1\n0 3\n4 2\n","output":"4\n1 5\n2 4 1\n1 2\n2 3 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSCC"}}}

use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponents;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let graph: Graph<Edge> = input.read();

    let (colors, condensed) = graph.strongly_connected_components();
    let mut res = vec![Vec::new(); condensed.vertex_count()];
    for (i, j) in colors.into_iter().enumerate().rev() {
        res[j].push(i);
    }
    out_line!(res.len());
    for v in res {
        out_line!(v.len(), v);
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
