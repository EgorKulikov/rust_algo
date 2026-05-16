//{"name":"E - MinCostFlow","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_e","interactive":false,"timeLimit":5000,"tests":[{"input":"3 1\n5 3 2\n1 4 8\n7 6 9\n","output":"19\nX..\n..X\n.X.\n"},{"input":"3 2\n10 10 1\n10 10 1\n1 1 10\n","output":"50\nXX.\nXX.\n..X\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMinCostFlow"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge_trait::FlowEdgeTrait;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::min_cost_flow::MinCostFlow;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let k: i64 = input.read();
    let a: Arr2d<i64> = input.read_table(n, n);

    let mut graph = Graph::new(2 * n + 2);
    for i in 0..n {
        graph.add_edge(2 * n, WeightedFlowEdge::new(i, 0i64, k));
        graph.add_edge(n + i, WeightedFlowEdge::new(2 * n + 1, 0i64, k));
        for j in 0..n {
            graph.add_edge(i, WeightedFlowEdge::new(j + n, -a[(i, j)], 1));
        }
    }
    let (ans, _) = graph.min_cost_flow(2 * n, 2 * n + 1);
    out_line!(-ans);
    let mut grid = Arr2d::new(n, n, '.');
    for i in 0..n {
        for e in graph[i].iter() {
            if e.to() >= n && e.to() < 2 * n && e.flow(&graph) > 0 {
                grid[(i, e.to() - n)] = 'X';
            }
        }
    }
    output().print_table(&grid);
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
