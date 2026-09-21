use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_flow_edge::WeightedFlowEdge;
use algo_lib::graph::min_cost_flow_slow::MinCostFlowSlow;
use algo_lib::graph::strongly_connected_components::StronglyConnectedComponentsTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_edges(n, &edges);
    let scc = graph.strongly_connected_components();
    let num = scc.condensed.vertex_count();
    let mut min_val = vec![i64::MAX; num];
    for i in 0..n {
        min_val[scc.color[i]].minim(a[i]);
    }
    for i in 0..n {
        if graph.degree(i) == 0 {
            min_val[scc.color[i]] = 0;
        }
    }
    let mut seen = BitSet::new(num);
    let mut graph = Graph::new_2d(2 * num + 2);
    let source = 2 * num;
    let sink = source + 1;
    for i in 0..num {
        graph.add_edge(WeightedFlowEdge::new(2 * i, sink, -min_val[i], 1));
        graph.add_edge(WeightedFlowEdge::new(2 * i, 2 * i + 1, 0, num as i64));
        graph.add_edge(WeightedFlowEdge::new(source, 2 * i + 1, 0, 1));
    }
    for i in 0..n {
        if min_val[scc.color[i]] == 0 {
            continue;
        }
        if a[i] != min_val[scc.color[i]] || seen[scc.color[i]] {
            graph.add_edge(WeightedFlowEdge::new(source, 2 * scc.color[i] + 1, a[i], 1));
        } else {
            seen.set(scc.color[i]);
        }
    }
    for (vert, e) in scc.condensed.edges() {
        graph.add_edge(WeightedFlowEdge::new(
            2 * vert + 1,
            2 * e.to(),
            0,
            num as i64,
        ));
    }
    let ans = a.copy_sum() - min_val.copy_sum() - graph.min_cost_flow_slow(source, sink).cost;
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
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
