//{"name":"H - Two SAT","group":"AtCoder - AtCoder Library Practice Contest","url":"https://atcoder.jp/contests/practice2/tasks/practice2_h","interactive":false,"timeLimit":5000,"tests":[{"input":"3 2\n1 4\n2 5\n0 6\n","output":"Yes\n4\n2\n0\n"},{"input":"3 3\n1 4\n2 5\n0 6\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HTwoSAT"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponents;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input) {
    let n = input.read();
    let d: i32 = input.read();
    let pos = input.read_table::<i32>(n, 2);

    let mut graph = Graph::new(2 * n);
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            for k in 0..2 {
                if (pos[(i, k)] - pos[(j, 0)]).abs() < d {
                    if (pos[(i, k)] - pos[(j, 1)]).abs() < d {
                        graph.add_edge(2 * i + k, Edge::new(2 * i + 1 - k));
                    } else {
                        graph.add_edge(2 * i + k, Edge::new(2 * j + 1));
                    }
                } else {
                    if (pos[(i, k)] - pos[(j, 1)]).abs() < d {
                        graph.add_edge(2 * i + k, Edge::new(2 * j));
                    }
                }
            }
        }
    }
    let (c, _) = graph.strongly_connected_components();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        if c[2 * i] == c[2 * i + 1] {
            out_line!("No");
            return;
        }
        res.push(pos[i][if c[2 * i] > c[2 * i + 1] { 0 } else { 1 }]);
    }
    out_line!("Yes");
    output().print_per_line(res.as_slice());
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
