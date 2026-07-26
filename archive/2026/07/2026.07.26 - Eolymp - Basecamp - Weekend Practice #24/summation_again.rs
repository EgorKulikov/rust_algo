use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::segment_tree::SegmentTreeNode;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::path_segment_tree::PathSegmentTreeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_unsigned_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut and = vec![0; n];
    let mut p = vec![0; n];
    let mut ans = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        and[vert] = a[vert];
        p[vert] = prev;
        for e in graph.adj(vert) {
            if e.to() == prev {
                continue;
            }
            and[vert] &= f.call(e.to(), vert);
        }
        ans += and[vert] as usize;
        and[vert]
    });
    dfs.call(0, n);

    let mut q = Arr2d::with_gen(n, 20, |v, i| {
        let mut qty = 0;
        if !a[v].is_set(i) {
            qty += 1;
        }
        for e in graph.adj(v) {
            if e.to() != p[v] && !and[e.to()].is_set(i) {
                qty += 1;
            }
        }
        qty
    });

    let qq = input.read_size();
    for _ in 0..qq {
        let v = input.read_size() - 1;
        let x = input.read_unsigned();
        for i in 0..20 {
            if x.is_set(i) {
                if a[v].is_set(i) {
                    let mut v = v;
                    loop {
                        q[(v, i)] += 1;
                        if q[(v, i)] == 1 {
                            ans -= 1 << i;
                        } else {
                            break;
                        }
                        v = p[v];
                        if v == n {
                            break;
                        }
                    }
                } else {
                    let mut v = v;
                    loop {
                        q[(v, i)] -= 1;
                        if q[(v, i)] == 0 {
                            ans += 1 << i;
                        } else {
                            break;
                        }
                        v = p[v];
                        if v == n {
                            break;
                        }
                    }
                }
                a[v].flip_bit(i);
            }
        }
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
