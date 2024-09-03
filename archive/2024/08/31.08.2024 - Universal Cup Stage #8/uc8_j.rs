//{"name":"uc8_j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"uc8_j"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n);
    for &(u, v, w) in &edges {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let spanning_tree = graph.minimal_spanning_tree();
    if spanning_tree.edge_count() != n - 1 {
        out.print_line((-1, -1));
        return;
    }
    let mut weight = 0;
    let mut max_odd = vec![vec![0; n]; 20];
    let mut max_even = vec![vec![0; n]; 20];
    let mut par = vec![vec![0; n]; 20];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        par[0][vert] = prev;
        for e in &spanning_tree[vert] {
            if e.to() == prev {
                continue;
            }
            weight += e.weight();
            if e.weight() % 2 == 0 {
                max_even[0][e.to()] = e.weight();
            } else {
                max_odd[0][e.to()] = e.weight();
            }
            f.call(e.to(), vert);
        }
    });
    dfs.call(0, 0);
    for i in 1..20 {
        for j in 0..n {
            par[i][j] = par[i - 1][par[i - 1][j]];
            max_even[i][j] = max_even[i - 1][j].max(max_even[i - 1][par[i - 1][j]]);
            max_odd[i][j] = max_odd[i - 1][j].max(max_odd[i - 1][par[i - 1][j]]);
        }
    }
    let lca = spanning_tree.lca();
    let mut other_weight = None;
    for (u, v, w) in edges {
        let cur = if w % 2 == 0 { &max_odd } else { &max_even };
        let l = lca.lca(u, v);
        for mut v in [u, v] {
            for i in (0..20).rev() {
                if lca.level(par[i][v]) >= lca.level(l) {
                    let cur = cur[i][v];
                    if cur != 0 {
                        other_weight.minim(weight + w - cur);
                    }
                    v = par[i][v];
                }
            }
        }
    }
    if weight % 2 == 0 {
        out.print_line((weight, other_weight));
    } else {
        out.print_line((other_weight, weight));
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
