//{"name":"C. Построение множеств","group":"Codeforces - Pinely Round 1 (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1761/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n0001\n1001\n0001\n0000\n3\n011\n001\n000\n","output":"3 1 2 3\n2 1 3\n2 2 4\n4 1 2 3 4\n1 1\n2 1 2\n3 1 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPostroenieMnozhestv"}}}

use algo_lib::collections::arr2d::Arr2dRead;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::topological_sort::TopologicalSort;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let b = input.read_table::<char>(n, n);

    let mut graph = Graph::new(n);
    for i in 0..n {
        for j in 0..n {
            if b[(i, j)] == '1' {
                graph.add_edge(i, Edge::new(j));
            }
        }
    }
    let order = graph.topological_sort().unwrap();
    let mut ans = vec![HashSet::new(); n];
    for i in order {
        ans[i].insert(i + 1);
        let row = ans[i].iter().copied().collect_vec();
        for e in &graph[i] {
            ans[e.to()].extend(row.iter().copied());
        }
    }
    for i in 0..n {
        let mut row = ans[i].iter().copied().collect_vec();
        row.sort();
        out_line!(ans[i].len(), row);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
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
