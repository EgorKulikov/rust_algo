use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let r = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let c = input.read_size_vec(q).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut times = Arr2d::new(n, r + 1, 0);
    let mut vis = vec![0; n];
    let mut ans = 0;
    for i in c {
        let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, rem: usize| {
            vis[vert] += 1;
            if vis[vert] == 1 {
                ans += 1;
            } else if vis[vert] == 2 {
                ans -= 1;
            }
            times[(vert, rem)] += 1;
            if times[(vert, rem)] >= 3 || rem == 0 {
                return;
            }
            for e in graph.adj(vert) {
                if e.to() == prev {
                    continue;
                }
                f.call(e.to(), vert, rem - 1);
            }
        });
        dfs.call(i, i, r);
        out.print_line(ans);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
