//{"name":"E. Edges, Colors and MST","group":"Yandex - Stage 10: Grand Prix of Kyoto","url":"https://official.contest.yandex.ru/opencupXXII/contest/35263/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 2 0\n2 3 1\n3 4 1\n2 4 0\n1 3 1\n","output":"3 1 4 5 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEdgesColorsAndMST"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges: Vec<(usize, usize, usize)> = input.read_vec(m);

    let mut graph = Graph::new(n);
    for (i, &(a, b, c)) in edges.iter().enumerate() {
        if c == 1 {
            graph.add_edge(a - 1, BiWeightedEdge::new(b - 1, i));
        }
    }
    let lca = graph.lca();
    let mut dsu = DSU::new(n);
    let mut highest = (0..n).collect_vec();
    let mut ans = vec![0; m];
    let mut next = 1;
    for i in 0..m {
        if ans[i] != 0 {
            continue;
        }
        let mut path = Vec::new();
        let (mut a, mut b, _) = edges[i];
        a = highest[dsu.get(a - 1)];
        b = highest[dsu.get(b - 1)];
        let c = highest[dsu.get(lca.lca(a, b))];
        for (mut a, c) in [(a, c), (b, c)] {
            while a != c {
                for e in &graph[a] {
                    if lca.level(e.to()) < lca.level(a) {
                        path.push(e.weight());
                        let b = highest[dsu.get(e.to())];
                        dsu.join(a, b);
                        highest[dsu.get(a)] = b;
                        a = b;
                        break;
                    }
                }
            }
        }
        path.sort_unstable();
        for j in path {
            assert_eq!(ans[j], 0);
            ans[j] = next;
            next += 1;
        }
        if ans[i] == 0 {
            ans[i] = next;
            next += 1;
        }
    }
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
