//{"name":"D: Treehouse Telegram","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-3/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n6\n1 2\n1 3\n2 4\n2 5\n3 6\n5\n1 2\n1 3\n1 4\n1 5\n2\n1 2\n10\n1 5\n5 2\n6 7\n9 7\n3 4\n2 4\n2 8\n10 3\n7 2\n","output":"Case #1: 23 8 1 0 0 0\nCase #2: 14 2 0 0 0\nCase #3: 1 0\nCase #4: 79 24 10 2 4 0 0 0 0 0\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"treehouse_telegram_.*input[.]txt"},"output":{"type":"file","fileName":"treehouse_telegram_output.txt","pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    drop(input);

    let graph = Graph::with_biedges(n, &edges);
    let lca = graph.lca();
    let mut ans = vec![0; n];
    for i in (1..=n).rev() {
        if i * i > n {
            for a in (i..=n).step_by(i) {
                for b in (i..a).step_by(i) {
                    ans[i - 1] += lca.path_length(a - 1, b - 1);
                }
            }
        } else {
            let mut dfs =
                RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (usize, usize) {
                    let mut sum = 0;
                    let mut qty = if (vert + 1) % i == 0 { 1 } else { 0 };
                    for e in &graph[vert] {
                        if e.to() == prev {
                            continue;
                        }
                        let (call_sum, call_qty) = f.call(e.to(), vert);
                        ans[i - 1] += call_sum * qty + call_qty * sum;
                        sum += call_sum;
                        qty += call_qty;
                    }
                    sum += qty;
                    (sum, qty)
                });
            dfs.call(0, n);
        }
        for j in (2 * i..=n).step_by(i) {
            ans[i - 1] -= ans[j - 1];
        }
    }
    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;
pub static TEST_TYPE: TestType = TestType::MultiNumber;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
    eprint!("\x1B[0m");
    output.flush();
    is_exhausted
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let paths = std::fs::read_dir(".").unwrap();
    let mut result = None;
    let mut last_accessed = None;
    let re = regex::Regex::new("treehouse_telegram_.*input[.]txt").unwrap();
    for path in paths {
        let path = path.unwrap();
        let cur_accessed = path.metadata().unwrap().accessed().unwrap();
        let path = path.path();
        let cur_name = path.file_name().unwrap().to_str().unwrap();
        if re.is_match(cur_name) {
            if last_accessed.is_none() || cur_accessed > last_accessed.unwrap() {
                result = Some(cur_name.to_string());
                last_accessed = Some(cur_accessed);
            }
        }
    }
    let in_file = std::fs::File::open(result.unwrap()).unwrap();
    let input = algo_lib::io::input::Input::file(in_file);
    let out_file = std::fs::File::create("treehouse_telegram_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
