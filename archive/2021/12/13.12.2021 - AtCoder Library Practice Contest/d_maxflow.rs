//{"name":"D - Maxflow","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_d","interactive":false,"timeLimit":5000,"tests":[{"input":"3 3\n#..\n..#\n...\n","output":"3\n#><\nvv#\n^^.\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaxflow"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::dirs::D4;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let m = input.read();
    let mut s: Arr2d<char> = input.read_table(n, m);

    let mut graph = Graph::new(n * m + 2);
    for i in 0..n {
        for j in ((i % 2)..m).step_by(2) {
            if s[(i, j)] == '.' {
                graph.add_edge(n * m, FlowEdge::new(i * m + j, 1));
                for (r, c) in D4::iter(i, j, n, m) {
                    if s[(r, c)] == '.' {
                        graph.add_edge(i * m + j, FlowEdge::new(r * m + c, 1));
                    }
                }
            }
        }
        for j in ((1 - i % 2)..m).step_by(2) {
            if s[(i, j)] == '.' {
                graph.add_edge(i * m + j, FlowEdge::new(n * m + 1, 1));
            }
        }
    }
    let ans = graph.max_flow(n * m, n * m + 1);
    for i in 0..n {
        for j in ((i % 2)..m).step_by(2) {
            for e in graph[i * m + j].iter() {
                if e.flow(&graph) > 0 && e.to() < n * m {
                    let r = e.to() / m;
                    let c = e.to() % m;
                    let (x, y) = if r > i {
                        ('v', '^')
                    } else if r < i {
                        ('^', 'v')
                    } else if c < j {
                        ('<', '>')
                    } else {
                        ('>', '<')
                    };
                    s[(i, j)] = x;
                    s[(r, c)] = y;
                }
            }
        }
    }
    out_line!(ans);
    output().print_table(&s);
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
