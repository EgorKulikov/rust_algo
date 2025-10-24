//{"name":"A: Warm Up","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/practice-round/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n5\n1 2 3 4 5\n1 2 3 4 5\n3\n1 1 2\n2 2 2\n4\n1 2 3 4\n3 4 4 4\n4\n1 2 3 4\n1 2 3 3\n3\n1 3 3\n2 2 2\n2\n1 2\n2 1\n","output":"Case #1: 0\nCase #2: 2\n3 1\n3 2\nCase #3: 3\n3 1\n4 2\n4 3\nCase #4: -1\nCase #5: -1\nCase #6: -1\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"warm_up_.*input[.]txt"},"output":{"type":"file","fileName":"warm_up_output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::output;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);
    drop(input);

    let mut id = FxHashMap::default();
    for i in 0..n {
        id.insert(a[i], i);
    }
    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| b[i]);
    let mut ans = Vec::new();
    for i in order {
        if a[i] > b[i] {
            output!(out, "Case #{}: -1", test_case);
            return;
        }
        if a[i] < b[i] {
            if let Some(&id) = id.get(&b[i]) {
                ans.push((id, i));
            } else {
                output!(out, "Case #{}: -1", test_case);
                return;
            }
        }
    }
    out.print_line((format!("Case #{}:", test_case), ans.len()));
    out.print_per_line(&ans.inc());
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, true, pre_calc, solve);
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
    let re = regex::Regex::new("warm_up(_validation)?_input.txt").unwrap();
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
    let out_file = std::fs::File::create("warm_up.out").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
