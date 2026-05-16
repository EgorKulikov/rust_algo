//{"name":"E. Запросы об остовном дереве","group":"Codeforces - Educational Codeforces Round 122 (Rated for Div. 2)","url":"https://codeforces.com/contest/1633/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"5 8\n4 1 4\n3 1 0\n3 5 3\n2 5 4\n3 4 8\n4 3 4\n4 2 8\n5 3 9\n3 11 1 1 10\n0 1 2\n","output":"4\n"},{"input":"6 7\n2 4 0\n5 4 7\n2 4 0\n2 1 7\n2 6 1\n3 4 4\n1 4 8\n4 10 3 3 7\n3 0 2 1\n","output":"5\n"},{"input":"3 3\n1 2 50\n2 3 100\n1 3 150\n1 10000000 0 0 100000000\n75\n","output":"164\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EZaprosiObOstovnomDereve"}}}

use algo_lib::collections::vec_ext::Bounds;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::out_line;
use std::iter::from_fn;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let edges: Vec<(usize, usize, i64)> = input.read_vec(m);
    let p = input.read_usize();
    let k = input.read_usize();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let q = input.read_long_vec(p);

    let mut weights = vec![0];
    for &(_, _, w) in &edges {
        weights.push(w);
        for &(_, _, v) in &edges {
            weights.push((w + v) / 2 + 1);
        }
    }
    weights.sort_unstable();
    weights.dedup();
    let mut f = Vec::with_capacity(weights.len());
    for &d in &weights {
        let mut graph = Graph::new(n);
        for &(u, v, w) in &edges {
            graph.add_edge(u - 1, BiWeightedEdge::new(v - 1, ((w - d).abs(), w)));
        }
        let tree = graph.minimal_spanning_tree();
        let mut k = 0i64;
        let mut b = 0;
        for i in 0..n {
            for e in &tree[i] {
                if i < e.to() {
                    if e.weight().1 <= d {
                        k += 1;
                        b -= e.weight().1;
                    } else {
                        k -= 1;
                        b += e.weight().1;
                    }
                }
            }
        }
        f.push((k, b));
    }
    let mut next = q[p - 1];
    let q = q.into_iter().chain(
        from_fn(|| {
            next = (a * next + b) % c;
            Some(next)
        })
        .take(k - p),
    );
    let ans = q.fold(0, |ans, q| {
        let i = weights.upper_bound(&q) - 1;
        let (k, b) = f[i];
        ans ^ (k * q + b)
    });
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
