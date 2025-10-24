//{"name":"L. Bot Friends 2","group":"Universal Cup - The 3rd Universal Cup Semifinals","url":"https://contest.ucup.ac/contest/2506/problem/14025","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 4\n2 3 7 1\n1 2 3\n1 3 1\n2 3 2\n3 4 2\n5 4\n100000 100000 100000 100000 1\n1 2 10\n2 3 100\n3 4 1000\n4 5 10000\n1 0\n1000000000\n","output":"12\n43214\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut a = input.read_long_vec(n);
    let edges: Vec<(usize, usize, i64)> = input.read_vec(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in edges.copy_iter() {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let mut heap = IndexedHeap::new(n);
    for i in 0..n {
        heap.add_or_adjust(i, a[i]);
    }
    let mut done = BitSet::new(n);
    while let Some((v, cost)) = heap.pop() {
        a[v] = cost;
        done.set(v);
        for e in &graph[v] {
            let to = e.to();
            let n_cost = cost + e.weight();
            if !done[to] {
                heap.add_or_relax(to, n_cost);
            }
        }
    }
    let mut n_graph = Graph::new(n);
    for (u, v, w) in edges {
        n_graph.add_edge(BiWeightedEdge::new(u, v, w + a[u] + a[v]));
    }
    let tree = n_graph.minimal_spanning_tree();
    let mut ans = -a.copy_sum();
    for i in 0..n {
        for e in &tree[i] {
            if e.to() > i {
                ans += e.weight();
            }
        }
    }
    out.print_line(ans + a.copy_min());
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
