//{"name":"Special Edge","group":"HackerEarth - April Circuits '23","url":"https://www.hackerearth.com/challenges/competitive/april-circuits-23/algorithm/special-edge-3-1e932e1e/","interactive":false,"timeLimit":1000,"tests":[{"input":"3 2\n4 1 3\n1 2\n2 3\n","output":"1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SpecialEdge"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::bridges::BridgeSearch;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(m).dec_by_one();

    let mut graph = Graph::new(n);
    for &(u, v) in &edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let bridges = graph.bridges();
    let bridges_set = bridges
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut dsu = DSU::new(n);
    for &(u, v) in &edges {
        if !bridges_set.contains(&(u, v)) && !bridges_set.contains(&(v, u)) {
            dsu.join(u, v);
        }
    }
    let mut a_cond = vec![0; n];
    for i in 0..n {
        a_cond[dsu.get(i)] += a[i];
    }
    let mut graph_cond = Graph::new(n);
    for &(u, v) in &bridges {
        let u = dsu.get(u);
        let v = dsu.get(v);
        graph_cond.add_edge(u, BiEdge::new(v));
    }
    let mut a_cum = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        a_cum[vert] = a_cond[vert];
        for e in &graph_cond[vert] {
            if e.to() == prev {
                continue;
            }
            a_cum[vert] += f.call(e.to(), vert);
        }
        a_cum[vert]
    });
    dfs.call(dsu.get(0), n);
    let mut ans = (n + 1, n + 1);
    let mut res = 0;
    let sum = a.iter().sum::<i64>();
    for (u, v) in bridges {
        let ou = u;
        let ov = v;
        let u = dsu.get(u);
        let v = dsu.get(v);
        let a = a_cum[u].min(a_cum[v]);
        let cand = a * (sum - a);
        if res.maxim(cand) {
            ans = (ou.min(ov) + 1, ou.max(ov) + 1);
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
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
