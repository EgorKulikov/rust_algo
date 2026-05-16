//{"name":"T637350 「Diligent-OI R2 C」所谓伊人","group":"Luogu","url":"https://www.luogu.com.cn/problem/T637350?contestId=241168","interactive":false,"timeLimit":1000,"tests":[{"input":"6 5\n1 1 4 5 1 4\n1 2\n2 1\n3 4\n4 5\n3 5\n","output":"0 0 1 0 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::strongly_connected_components::{
    StronglyConnectedComponents, StronglyConnectedComponentsTrait,
};
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let p = input.read_int_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_edges(n, &edges);
    let mut dsu = DSU::new(n);
    for (u, v) in edges {
        dsu.union(u, v);
    }
    let mut weight = vec![0; n];
    for i in 0..n {
        weight[dsu.find(i)].maxim(p[i]);
    }
    let StronglyConnectedComponents { color, condensed } = graph.strongly_connected_components();
    let mut ans = vec![usize::MAX; n];
    for i in 0..n {
        if p[i] == weight[dsu.find(i)] {
            ans[i] = 0;
        }
    }
    let mut wg = Graph::new(2 * condensed.vertex_count() + 1);
    let root = 2 * condensed.vertex_count();
    for i in 0..condensed.vertex_count() {
        wg.add_edge(WeightedEdge::new(i, i + condensed.vertex_count(), 1));
        wg.add_edge(WeightedEdge::new(i + condensed.vertex_count(), i, 1));
        for e in &condensed[i] {
            wg.add_edge(WeightedEdge::new(i, e.to(), 0));
            wg.add_edge(WeightedEdge::new(
                e.to() + condensed.vertex_count(),
                i + condensed.vertex_count(),
                0,
            ));
        }
    }
    for i in 0..n {
        if ans[i] == 0 {
            wg.add_edge(WeightedEdge::new(root, color[i], 1));
            wg.add_edge(WeightedEdge::new(
                root,
                color[i] + condensed.vertex_count(),
                1,
            ));
        }
    }
    let dist = wg.zero_one_distances_from(root);
    for i in 0..n {
        ans[i].minim(dist[color[i]].unwrap().0);
        ans[i].minim(dist[color[i] + condensed.vertex_count()].unwrap().0);
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
