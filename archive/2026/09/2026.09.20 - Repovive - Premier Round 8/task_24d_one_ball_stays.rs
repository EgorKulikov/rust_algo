use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    if a == b {
        out.print_line(true);
        return;
    }
    if a.copy_sum() != b.copy_sum() {
        out.print_line(false);
        return;
    }
    let mut bad_a = true;
    let mut bad_b = true;
    for (u, v) in edges.copy_iter() {
        if a[u] + a[v] > 1 {
            bad_a = false;
        }
        if b[u] + b[v] > 1 {
            bad_b = false;
        }
    }
    if bad_a || bad_b {
        out.print_line(false);
        return;
    }
    let graph = Graph::with_biedges(n, &edges);
    let mut col = vec![0; n];
    let mut rec = RecursiveFunction2::new(|rec, v: usize, c: i32| -> bool {
        if col[v] != 0 {
            col[v] == c
        } else {
            col[v] = c;
            for e in graph.adj(v) {
                if !rec.call(e.to(), -c) {
                    return false;
                }
            }
            true
        }
    });
    if !rec.call(0, 1) {
        out.print_line(true);
        return;
    }
    let mut qa = 0;
    let mut qb = 0;
    for i in 0..n {
        if a[i] % 2 == 1 {
            if col[i] == 1 {
                qa += 1;
            } else {
                qa -= 1;
            }
        }
        if b[i] % 2 == 1 {
            if col[i] == 1 {
                qb += 1;
            } else {
                qb -= 1;
            }
        }
    }
    out.print_line(qa == qb);
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
