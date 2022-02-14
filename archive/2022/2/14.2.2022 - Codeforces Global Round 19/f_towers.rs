//{"name":"F. Towers","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 1\n1 2\n2 3\n","output":"4\n"},{"input":"5\n1 3 3 1 3\n1 3\n5 4\n4 3\n2 3\n","output":"7\n"},{"input":"2\n6 1\n1 2\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTowers"}}}

use algo_lib::collections::iter_ext::IterPartialEqExt;
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
    let h = input.read_long_vec(n);
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let max_h = *h.iter().max().unwrap();
    let root = h.iter().find(&max_h).unwrap();
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert, prev| {
        let mut heights = Vec::new();
        for e in &graph[vert] {
            if e.to() != prev {
                heights.push(f.call(e.to(), vert));
            }
        }
        heights.sort_unstable();
        heights.reverse();
        if heights.is_empty() {
            ans += h[vert];
            h[vert]
        } else if vert != prev {
            if h[vert] > heights[0] {
                ans += h[vert] - heights[0];
                h[vert]
            } else {
                heights[0]
            }
        } else {
            assert!(heights.len() >= 1);
            if heights.len() == 1 {
                heights.push(0);
            }
            ans += 2 * h[vert] - heights[0] - heights[1];
            h[vert]
        }
    });
    dfs.call(root, root);
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::Single;
    match test_type {
        TestType::Single => solve(&mut input, 1),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, i + 1);
            }
        }
        TestType::MultiEof => {
            let mut i = 1usize;
            while input.peek().is_some() {
                solve(&mut input, i);
                i += 1;
            }
        }
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
