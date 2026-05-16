//{"name":"Two Trees","group":"HackerEarth - March Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/march-circuits-23/algorithm/two-trees-ee18cad2/","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 2 42\n2 4 11\n4 6 11\n1 3 50\n3 5 39\n29 25 8 49 13 9\n3\n5 3\n6 2\n6 1\n","output":"60\n44\n102\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TwoTrees"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable5, RecursiveFunction5};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(n - 1);
    let w = input.read_long_vec(n);

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(u - 1, BiWeightedEdge::new(v - 1, w));
    }
    let lca = graph.lca();
    let mut parent = Arr2d::new(n, 20, 0);
    let mut path = Arr2d::new(n, 20, 0);
    let mut back = Arr2d::new(n, 20, 0);
    back[(0, 0)] = w[0];
    let mut sw_at = vec![0; n];
    let mut ltr = vec![0; n];
    let mut dfs = RecursiveFunction5::new(
        |f,
         vert: usize,
         prev: usize,
         mut switch_at: usize,
         mut cost_to_root: i64,
         length_to_root: i64| {
            ltr[vert] = length_to_root;
            if cost_to_root.minim(length_to_root + w[vert]) {
                switch_at = vert;
            }
            sw_at[vert] = switch_at;
            parent[(vert, 0)] = prev;
            for i in 1..20 {
                let p = parent[(parent[(vert, i - 1)], i - 1)];
                parent[(vert, i)] = p;
                path[(vert, i)] = path[(vert, i - 1)] + path[(parent[(vert, i - 1)], i - 1)];
                back[(vert, i)] = (back[(vert, i - 1)] + 2 * path[(parent[(vert, i - 1)], i - 1)])
                    .min(path[(vert, i - 1)] + back[(parent[(vert, i - 1)], i - 1)]);
            }

            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                path[(e.to(), 0)] = e.weight();
                back[(e.to(), 0)] = (w[e.to()] + 2 * e.weight()).min(w[vert] + e.weight());
                f.call(
                    e.to(),
                    vert,
                    switch_at,
                    cost_to_root + e.weight() * 2,
                    length_to_root + e.weight(),
                );
            }
        },
    );
    dfs.call(0, 0, 0, w[0], 0);

    let q = input.read_size();
    for _ in 0..q {
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;

        let mut ans = 2 * (ltr[u] - ltr[v]);
        let p = sw_at[u];
        if lca.level(p) <= lca.level(v) {
            out_line!(ans);
            continue;
        }
        let mut res = None;
        let mut pos = p;
        let mut add = 0;
        for i in (0..20).rev() {
            if lca.level(pos) - lca.level(v) >= (1 << i) {
                let np = parent[(pos, i)];
                let cur = add + back[(pos, i)] + (ltr[np] - ltr[v]) * 2;
                res.minim(cur);
                add += ltr[pos] - ltr[np];
                pos = np;
            }
        }
        ans.minim(2 * (ltr[u] - ltr[p]) + w[p] + res.unwrap());
        out_line!(ans);
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
