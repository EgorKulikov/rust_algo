//{"name":"Oh, There You Are, Perry","group":"CodeChef - STRV2022","url":"https://www.codechef.com/STRV2022/problems/AGENTPERRY","interactive":false,"timeLimit":1000,"tests":[{"input":"7 4\n4\n1 3\n1 4\n1 7\n4 2\n7 5\n6 3\n2 3 1 1 4 2 3\n3\n3 1\n1 3\n5 2\n","output":"3\n7\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"OhThereYouArePerry"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::out_line;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read_usize();
    let m = input.read_usize();
    let root = input.read_usize() - 1;
    let edges = input.read_usize_pair_vec(n - 1).dec_by_one();
    let tp = input.read_usize_vec(n).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let mut pos = Arr2d::new(n, m, None);
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        pos[(vert, tp[vert])] = Some(vert);
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert);
            for i in 0..m {
                if let Some(x) = pos[(e.to(), i)] {
                    pos[(vert, i)].minim(x);
                }
            }
        }
    });
    dfs.call(root, n);
    let lca = graph.lca_with_root(root);

    let q = input.read_usize();
    for _ in 0..q {
        let mut u = input.read_usize() - 1;
        let typ = input.read_usize() - 1;
        if pos[(u, typ)].is_none() {
            for j in (0..20).rev() {
                let p = lca.predecessor(j, u);
                if p.is_some() && pos[(p.unwrap(), typ)].is_none() {
                    u = p.unwrap();
                }
            }
            if u == root {
                out_line!(-1);
                continue;
            }
            u = lca.parent(u).unwrap();
        }
        out_line!(pos[(u, typ)].unwrap() + 1);
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
