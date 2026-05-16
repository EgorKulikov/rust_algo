//{"name":"F. Дореми и экспериментальное дерево","group":"Codeforces - Codeforces Global Round 24","url":"https://codeforces.com/contest/1764/problem/F","interactive":false,"timeLimit":2500,"tests":[{"input":"3\n7\n3 5\n0 2 8\n","output":"2 3 3\n1 2 2\n"},{"input":"9\n8081910646\n8081902766 8081903751\n8081902555 8081903540 8081905228\n3090681001 3090681986 3090681775 7083659398\n7083657913 7083658898 7083658687 2092437133 15069617722\n1748216295 1748217280 1748217069 5741194692 749972427 10439821163\n2377558289 2377559274 2377559063 6370536686 1379314421 5028071980 8866466178\n1746983932 1746984917 1746984706 5739962329 748740064 10438588800 5026839617 10448447704\n2341942133 2341943118 2341942907 6334920530 1343698265 4992455824 8830850022 4991223461 9115779270\n","output":"1 2 985\n2 3 211\n2 4 998244353\n2 5 998244853\n4 6 671232353\n6 8 1232363\n4 7 356561356\n7 9 35616156\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDoremiIEksperimentalnoeDerevo"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let mut d = Arr2d::new(n, n, 0);
    for i in 0..n {
        for j in 0..=i {
            d[(i, j)] = input.read_long();
            d[(j, i)] = d[(i, j)];
        }
    }

    let mut graph = Graph::new(n);
    for i in 0..n {
        for j in 0..i {
            let d1 = d[(i, i)] - d[(i, j)];
            let d2 = d[(j, j)] - d[(i, j)];
            let sum_d = d1 + d2;
            if sum_d % n.into_i64() != 0 {
                continue;
            }
            let c = sum_d / n.into_i64();
            if d1 % c != 0 {
                continue;
            }
            if d2 % c != 0 {
                continue;
            }
            graph.add_edge(i, BiWeightedEdge::new(j, c));
        }
    }
    assert!(graph.is_connected());
    let graph = graph.minimal_spanning_tree();
    for i in 0..n {
        for e in &graph[i] {
            if e.to() < i {
                out_line!(i + 1, e.to() + 1, e.weight());
            }
        }
    }
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
