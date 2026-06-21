use algo_lib::collections::segment_tree::SegmentTreeNode;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::graph::path_segment_tree::PathSegmentTreeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let x = input.read_long();
    let mut edges = input.read_vec::<(usize, usize, i64)>(m);

    for (u, v, _) in edges.iter_mut() {
        *u -= 1;
        *v -= 1;
    }
    let mut graph = Graph::new(n, m);
    for (u, v, w) in edges.iter().copied() {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    let tree = graph.minimal_spanning_tree();
    let base = tree.edges().map(|(_, e)| e.weight()).sum::<i64>() / 2;
    let mut ans = None;
    if m > n - 1 {
        ans = Some(base);
    }
    #[derive(Default, Clone)]
    struct Node {
        val: i64,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.max(right_val.val);
        }
    }
    let mut to_par = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, w: i64| {
        to_par[vert] = w;
        for e in tree.adj(vert) {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, e.weight());
        }
    });
    dfs.call(0, 0, 0);
    let mut st = tree.path_segment_tree_with_gen(false, |i| Node { val: to_par[i] });

    for (u, v, w) in edges {
        let max = st.query(u..=v).val;
        if ans.is_none() {
            ans = Some(base - max + (w ^ x));
        } else {
            ans = Some(ans.unwrap().min(base - max + (w ^ x)));
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

    let t = input.read();
    for i in 1..=t {
        solve(&mut input, &mut output, i, &mut pre_calc);
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
