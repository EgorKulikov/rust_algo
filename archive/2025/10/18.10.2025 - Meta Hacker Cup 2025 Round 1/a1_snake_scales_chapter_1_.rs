//{"name":"A1: Snake Scales (Chapter 1)","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-1/problems/A1","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n5\n2 4 5 1 4\n3\n13 10 11\n4\n1 3 3 7\n1\n42\n3\n5 50 42\n7\n4 2 5 6 4 2 1\n","output":"Case #1: 4\nCase #2: 3\nCase #3: 4\nCase #4: 0\nCase #5: 45\nCase #6: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"snake_scales_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"snake_scales_chapter__output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIterCopy;
use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    drop(input);

    let ans = a
        .consecutive_iter_copy()
        .map(|(x, y)| (x - y).abs())
        .max()
        .unwrap_or(0);
    out.print_line((format!("Case #{}:", test_case), ans));
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
    let re = regex::Regex::new("snake_scales_chapter_.*_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("snake_scales_chapter__output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
