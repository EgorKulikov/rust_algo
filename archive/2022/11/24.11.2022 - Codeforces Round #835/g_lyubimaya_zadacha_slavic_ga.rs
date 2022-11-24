//{"name":"G. Любимая задача SlavicG-а","group":"Codeforces - Codeforces Round #835 (Div. 4)","url":"https://codeforces.com/contest/1760/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 1 4\n1 3 1\n2 3 2\n4 3 3\n3 5 1\n2 1 2\n1 2 2\n6 2 3\n1 2 1\n2 3 1\n3 4 1\n4 5 3\n5 6 5\n","output":"YES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GLyubimayaZadachaSlavicGA"}}}

use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::out_line;
use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let a = input.read_usize() - 1;
    let b = input.read_usize() - 1;
    let edges = input.read_vec::<(usize, usize, u32)>(n - 1);

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(u - 1, BiWeightedEdge::new(v - 1, w));
    }

    fn build_xors(
        graph: &Graph<BiWeightedEdge<u32>>,
        root: usize,
        forbidden: Option<usize>,
    ) -> Vec<u32> {
        let mut xors = vec![u32::MAX; graph.vertex_count()];
        let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, val: u32| {
            if forbidden == Some(vert) {
                return;
            }
            xors[vert] = val;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert, val ^ e.weight());
            }
        });
        dfs.call(root, root, 0);
        xors
    }

    let from_a = build_xors(&graph, a, Some(b));
    let from_b = build_xors(&graph, b, None);
    let mut good = HashSet::new();
    for i in 0..n {
        if i != b {
            good.insert(from_b[i]);
        }
    }
    for i in from_a {
        if good.contains(&i) {
            out_line!(true);
            return;
        }
    }
    out_line!(false);
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
