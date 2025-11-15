//{"name":"C: Designing Paths","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-2/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n7 2 2\n4 1 5 7 2\n3 2 3 4\n5 1 2\n3 1 2 3\n3 1 4 5\n5 1 1\n5 1 2 3 4 5\n5 4 1\n5 1 2 3 4 5\n5 1 1\n5 3 4 5 1 2\n","output":"Case #1: 31\nCase #2: 22\nCase #3: 40\nCase #4: 14\nCase #5: -10\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"designing_paths_.*input[.]txt"},"output":{"type":"file","fileName":"designing_paths_output.txt","pattern":null}}

use algo_lib::collections::indexed_heap::IndexedHeap;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::numbers::num_utils::UpperDiv;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();
    let mut paths = Vec::with_capacity(m);
    let mut total_length = 0;
    for _ in 0..m {
        let l = input.read_size();
        total_length += l;
        paths.push(input.read_size_vec(l).dec());
    }
    drop(input);

    let mut graph = Graph::new(n + total_length);
    let mut at = n;
    for path in paths {
        for i in 1..path.len() {
            graph.add_edge(WeightedEdge::new(at + i - 1, at + i, 1));
        }
        for i in 0..path.len() {
            graph.add_edge(WeightedEdge::new(path[i], at + i, 0));
            graph.add_edge(WeightedEdge::new(at + i, path[i], 0));
        }
        at += path.len();
    }
    let mut res = vec![None; graph.vertex_count()];
    let mut heap = IndexedHeap::new(graph.vertex_count());
    heap.add_or_adjust(0, 0);
    while let Some((cur, dist)) = heap.pop() {
        res[cur] = Some(dist);
        for e in &graph[cur] {
            let next = e.to();
            if res[next].is_some() {
                continue;
            }
            let mut total = dist + e.weight();
            if next < n {
                total = total.upper_div(k) * k;
            }
            let next_dist = total;
            heap.add_or_relax(next, next_dist);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if let Some(d) = res[i] {
            ans += ((d / k) * (i + 1)) as i64;
        } else {
            ans -= (i + 1) as i64;
        }
    }
    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

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
    let re = regex::Regex::new("designing_paths_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("designing_paths_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
