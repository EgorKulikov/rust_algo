//{"name":"D: Crash Course","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-1/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n7\nABBAAAB\n1\nA\n1\nB\n2\nAB\n6\nAAAAAA\n7\nBBBBBBA\n","output":"Case #1: Alice\nCase #2: Alice\nCase #3: Bob\nCase #4: Bob\nCase #5: Alice\nCase #6: Alice\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"crash_course_.*input[.]txt"},"output":{"type":"file","fileName":"crash_course_output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::string::str::StrReader;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    drop(input);

    let mut a = 0;
    let mut b = 0;
    for i in (0..n).rev() {
        if s[i] == b'A' {
            a += 1;
        } else {
            b += 1;
        }
        if a > b {
            out.print_line((format!("Case #{}:", test_case), "Alice"));
            return;
        }
    }
    out.print_line((format!("Case #{}:", test_case), "Bob"));
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
    let re = regex::Regex::new("crash_course_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("crash_course_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
