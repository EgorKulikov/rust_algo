//{"name":"Bi-directional roads","group":"HackerEarth - January Easy '22","url":"https://www.hackerearth.com/challenges/competitive/january-easy-22/algorithm/cities-in-distress-67ef9c04/","interactive":false,"timeLimit":5000,"tests":[{"input":"5 2\n1 2 5\n2 3 3\n4 1 2\n4 5 7\n3 2 3 5\n2 4 5\n","output":"1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BiDirectionalRoads"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
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
    let q = input.read_usize();
    let edges: Vec<(usize, usize, u64)> = input.read_vec(n - 1);

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(u - 1, BiWeightedEdge::new(v - 1, w));
    }
    let lca = graph.lca();
    let mut dist = vec![0; n];
    let mut dist_to_parent = vec![0; n];
    let mut order = (1..n).collect_vec();
    order.sort_by_key(|&i| lca.level(i));
    for i in order {
        let w = {
            let mut res = None;
            for e in graph[i].iter() {
                if Some(e.to()) == lca.parent(i) {
                    res = Some(e.weight());
                    break;
                }
            }
            res.unwrap()
        };
        dist_to_parent[i] = w;
        dist[i] = dist[lca.parent(i).unwrap()] + w;
    }
    let mut up = Arr2d::new(20, n, (0, 0));
    for i in 0..n {
        up[(0, i)] = (lca.parent(i).unwrap_or(0), dist_to_parent[i]);
    }
    for i in 1..up.d1() {
        for j in 0..n {
            let p = up[(i - 1, j)].0;
            up[(i, j)] = (up[(i - 1, p)].0, up[(i - 1, j)].1 + up[(i - 1, p)].1);
        }
    }

    let distance = |a, b| dist[a] + dist[b] - 2 * dist[lca.lca(a, b)];
    let find = |mut v, mut d| {
        for i in (0..up.d1()).rev() {
            let (to, dist) = up[(i, v)];
            if d >= dist {
                d -= dist;
                v = to;
            }
        }
        v
    };

    for _ in 0..q {
        let k = input.read_usize();
        let a = input.read_usize_vec(k).dec_by_one();
        let mut p = a[0];
        let mut ld = None;
        let mut lowest = 0;
        for &i in &a {
            p = lca.lca(p, i);
            if ld.maxim(dist[i]) {
                lowest = i;
            }
        }
        let mut left = 0;
        let mut right = u64::MAX / 2;
        while left < right {
            let mid = (left + right) >> 1;
            let vert = find(lowest, mid);
            if a.iter().any(|&v| distance(v, vert) > mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let mut ans = find(lowest, left);
        if left > 0 {
            let cand = find(lowest, left - 1);
            if !a.into_iter().any(|v| distance(v, cand) > left) {
                ans.minim(cand);
            }
        }
        out_line!(ans + 1);
    }
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
