//{"name":"Chain Reactions","group":"Google Coding Competitions - Qualification Round 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/0000000000876ff1/0000000000a45ef7","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n4\n60 20 40 50\n0 1 1 2\n5\n3 2 1 4 5\n0 1 1 1 0\n8\n100 100 100 90 80 100 90 100\n0 1 2 1 2 3 1 3\n","output":"Case #1: 110\nCase #2: 14\nCase #3: 490\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ChainReactions"}}}

use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::out_line;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.read_usize();
    let fun = input.read_long_vec(n);
    let p = input.read_usize_vec(n);

    let mut graph = Graph::new(n);
    for (i, &p) in p.iter().enumerate() {
        if p != 0 {
            graph.add_edge(p - 1, Edge::new(i));
        }
    }
    let mut ans = 0;
    for (i, p) in p.into_iter().enumerate() {
        if p == 0 {
            let mut dfs = RecursiveFunction::new(|f, vert| -> i64 {
                let mut calls = Vec::with_capacity(graph[vert].len());
                for e in &graph[vert] {
                    calls.push(f.call(e.to()));
                }
                calls.sort_unstable();
                if calls.is_empty() {
                    fun[vert]
                } else {
                    ans += calls.iter().skip(1).sum::<i64>();
                    fun[vert].max(calls[0])
                }
            });
            ans += dfs.call(i);
        }
    }
    out_line!(format!("Case #{}:", test_case), ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
