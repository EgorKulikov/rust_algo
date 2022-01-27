//{"name":"B. Building Routes","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/B/","interactive":false,"timeLimit":5000,"tests":[{"input":"5 4\n4 1 s\n3 2 n\n1 2 w\n5 4 s\n3 2\n1 3\n2 1\n1 4\n","output":"0\n3\n1\n0\n"},{"input":"10 8\n1 2 a\n2 3 b\n3 4 c\n4 5 d\n5 6 e\n6 7 f\n7 8 g\n8 9 h\n9 10 i\n2 3\n3 2\n10 1\n3 5\n7 8\n10 9\n5 4\n4 8\n","output":"1\n0\n8\n3\n6\n0\n0\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBuildingRoutes"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let q = input.read_usize();
    let edges: Vec<(usize, usize, char)> = input.read_vec(n - 1);

    let mut graph = Graph::new(n);
    for (a, b, c) in edges {
        graph.add_edge(
            a - 1,
            BiWeightedEdge::new(b - 1, (c as usize) - ('a' as usize)),
        );
    }
    let mut ans = Arr2d::new(n, n, 0);
    for i in 0..n {
        let mut res = 0;
        let mut dfs = RecursiveFunction::new(|f, edges: Vec<(usize, usize)>| {
            let mut next = Vec::new();
            for (vert, prev) in edges {
                for e in &graph[vert] {
                    if e.to() != prev {
                        next.push((e.weight(), e.to(), vert));
                    }
                }
            }
            next.sort();
            let mut last = 26;
            let mut n_res = res;
            let mut call = Vec::new();
            for (w, vert, prev) in next {
                if w != last {
                    res = n_res;
                    f.call(call);
                    call = Vec::new();
                    last = w;
                    n_res = res;
                }
                ans[(i, vert)] = res;
                call.push((vert, prev));
                n_res += 1;
            }
            if !call.is_empty() {
                res = n_res;
                f.call(call);
            }
        });
        dfs.call(vec![(i, i)]);
    }
    for _ in 0..q {
        let s = input.read_usize() - 1;
        let f = input.read_usize() - 1;
        out_line!(ans[(s, f)]);
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
