use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::UpperDiv;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    for i in 0..n {
        if a[i].upper_div(2) != b[i].upper_div(2) {
            out.print_line(false);
            return;
        }
    }
    let mut graph = Graph::new(n, m);
    for (u, v) in edges {
        if a[u] != 0 && a[v] != 0 {
            graph.add_edge(BiEdge::new(u, v));
        }
    }
    let mut seen = BitSet::new(n);
    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut dfs = RecursiveFunction::new(|f, vert: usize| -> i32 {
            if seen[vert] {
                return 0;
            }
            seen.set(vert);
            let mut res = a[vert] - b[vert];
            for e in graph.adj(vert) {
                res += f.call(e.to());
            }
            res
        });
        if dfs.call(i) != 0 {
            out.print_line(false);
            return;
        }
    }
    out.print_line(true);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
