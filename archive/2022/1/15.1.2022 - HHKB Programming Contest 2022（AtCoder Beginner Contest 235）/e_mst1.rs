//{"name":"E - MST + 1","group":"AtCoder - HHKB Programming Contest 2022（AtCoder Beginner Contest 235）","url":"https://atcoder.jp/contests/abc235/tasks/abc235_e","interactive":false,"timeLimit":4000,"tests":[{"input":"5 6 3\n1 2 2\n2 3 3\n1 3 6\n2 4 5\n4 5 9\n3 5 8\n1 3 1\n3 4 7\n3 5 7\n","output":"Yes\nNo\nYes\n"},{"input":"2 3 2\n1 2 100\n1 2 1000000000\n1 1 1\n1 2 2\n1 1 5\n","output":"Yes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMST1"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    let n = input.read_usize();
    let m = input.read_usize();
    let q = input.read_usize();

    let mut graph = Graph::new(n);
    for _ in 0..m {
        let (a, b, c): (usize, usize, u32) = input.read();
        graph.add_edge(a - 1, BiWeightedEdge::new(b - 1, c));
    }
    let graph = graph.minimal_spanning_tree();
    let lca = graph.lca();
    let mut max_edge = Arr2d::new(lca.num_levels(), n, None);
    for i in 0..n {
        for e in &graph[i] {
            if Some(e.to()) == lca.parent(i) {
                max_edge[(0, i)] = Some(e.weight());
            }
        }
    }
    for j in 1..lca.num_levels() {
        for i in 0..n {
            let w1 = max_edge[(j - 1, i)];
            let p = lca.predecessor(j - 1, i);
            let res = match p {
                None => None,
                Some(p) => match max_edge[(j - 1, p)] {
                    None => None,
                    Some(w2) => Some(w1.unwrap().max(w2)),
                },
            };
            max_edge[(j, i)] = res;
        }
    }
    let weight = |mut f, t| {
        let delta = lca.level(f) - lca.level(t);
        let mut ans = 0;
        for i in 0..max_edge.d1() {
            if delta.is_set(i) {
                ans.maxim(max_edge[(i, f)].unwrap());
                f = lca.predecessor(i, f).unwrap();
            }
        }
        ans
    };
    for _ in 0..q {
        let u = input.read_usize() - 1;
        let v = input.read_usize() - 1;
        let w = input.read_unsigned();

        let l = lca.lca(u, v);
        if weight(v, l).max(weight(u, l)) > w {
            out_line!("Yes");
        } else {
            out_line!("No");
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
