//{"name":"F. Tree Walk","group":"Codeforces - GoForGold Long Challenge - May 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/611602/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"10 10\n1 3\n2 1\n3 1\n3 2\n5 2\n3 3\n7 3\n7 3\n7 1\n2\n5 7\n4\n10 6 7 1\n6\n5 8 3 6 9 2\n1\n5\n2\n2 1\n1\n10\n1\n6\n1\n10\n1\n9\n1\n10\n","output":"11\n16\n24\n6\n3\n8\n8\n8\n10\n8\n"},{"input":"5 3\n1 3\n2 1\n3 3\n4 1\n2\n5 1\n3\n2 4 5\n4\n1 3 5 2\n","output":"8\n8\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::dfs_order::DFSOrderTrait;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let edges = input.read_vec::<(usize, i64)>(n - 1);

    let mut graph = Graph::new(n);
    for i in 0..n - 1 {
        let (p, w) = edges[i];
        graph.add_edge(BiWeightedEdge::new(p - 1, i + 1, w));
    }
    let mut dist = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, d: i64| {
        dist[vert] = d;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, d + e.weight());
        }
    });
    dfs.call(0, n, 0);
    let dfs_order = graph.dfs_order().position;
    let lca = graph.lca();

    for _ in 0..q {
        let k = input.read_size();
        let vert = input
            .read_size_vec(k)
            .dec()
            .sorted_by_key(|x| dfs_order[*x]);
        let mut max = 0;
        let mut last = 0;
        let mut ans = 0;
        for i in vert {
            ans += dist[i] + dist[last] - 2 * dist[lca.lca(i, last)];
            max.maxim(dist[i]);
            last = i;
        }
        ans += dist[last];
        ans -= max;
        out.print_line(ans);
    }
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
