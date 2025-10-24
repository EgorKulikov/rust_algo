//{"name":"B1: Final Product (Chapter 1)","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-1/problems/B1","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n2 5 63\n2 6 12\n2 6 12\n1 100 9\n5 63 64\n","output":"Case #1: 1 3 3 7\nCase #2: 3 2 1 2\nCase #3: 2 1 1 6\nCase #4: 3 3\nCase #5: 1 4 1 1 1 1 16 1 1 1 1\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"final_product_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"final_product_chapter__output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let _a = input.read_int();
    let b = input.read_int();
    drop(input);

    let mut ans = vec![1; 2 * n];
    ans[Back(0)] = b;
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
    let re = regex::Regex::new("final_product_chapter_.*_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("final_product_chapter__output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
