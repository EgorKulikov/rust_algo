//{"name":"task_c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"task_c"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let edges = input.read_vec::<(usize, usize)>(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (i32, i32) {
        let mut ans_leaf = 1;
        let mut ans_non_leaf = 0;
        let mut best_delta = i32::MIN / 2;
        for e in graph[vert].iter() {
            if prev == e.to() {
                continue;
            }
            let (call_leaf, call_non_leaf) = f.call(e.to(), vert);
            best_delta.maxim(call_non_leaf - call_leaf);
            ans_leaf += call_leaf.max(call_non_leaf);
            ans_non_leaf += call_leaf.max(call_non_leaf);
        }
        if best_delta < 0 {
            ans_leaf += best_delta;
        }
        (ans_leaf, ans_non_leaf)
    });
    out_line!(dfs.call(0, 0).0);
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
