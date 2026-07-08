use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let target = b"KEPPY5";
    let graph = Graph::with_biedges(n, &edges);
    let mut ans = 0;
    let mut dfs =
        RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (Vec<usize>, Vec<usize>) {
            let mut up = vec![0; 7];
            let mut down = vec![0; 7];
            if s[vert] == target[0] {
                up[1] = 1;
            } else {
                up[0] = 1;
            }
            if s[vert] == target[5] {
                down[1] = 1;
            } else {
                down[0] = 1;
            }
            for e in graph.adj(vert) {
                if e.to() == prev {
                    continue;
                }
                let (call_up, call_down) = f.call(e.to(), vert);
                for i in 0..=6 {
                    for j in 6 - i..=6 {
                        ans += down[i] * call_up[j] + up[i] * call_down[j];
                    }
                }
                for i in 0..=6 {
                    if i < 6 && s[vert] == target[i] {
                        up[i + 1] += call_up[i];
                    } else {
                        up[i] += call_up[i];
                    }
                    if i < 6 && s[vert] == target[5 - i] {
                        down[i + 1] += call_down[i];
                    } else {
                        down[i] += call_down[i];
                    }
                }
            }
            (up, down)
        });
    dfs.call(0, n);
    out.print_line(ans);
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
