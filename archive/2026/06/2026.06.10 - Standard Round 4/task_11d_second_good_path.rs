use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdgeWithId;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let mut graph = Graph::new(n, m);
    for (u, v) in edges {
        graph.add_edge(BiEdgeWithId::new(u, v));
    }
    let d = graph.edge_distances(0);

    let mut first_sum = vec![None; n];
    let mut second_sum = vec![None; n];
    let mut rec = RecursiveFunction2::new(|rec, vert: usize, sum: usize| {
        if second_sum[vert].is_some() {
            return;
        }
        if first_sum[vert].is_none() {
            first_sum[vert] = Some(sum);
        } else {
            second_sum[vert] = Some(sum);
        }
        let mut edges = Vec::new();
        for e in graph.adj(vert) {
            if d[e.to()] == d[vert] + 1 {
                edges.push((e.id(), e.to()));
            }
        }
        edges.sort();
        for (id, to) in edges {
            rec.call(to, sum + id + 1);
        }
    });
    rec.call(0, 0);
    out.print_line(second_sum);
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
