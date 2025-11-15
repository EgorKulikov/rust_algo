//{"name":"A: Deciding Points","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-2/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n4 3\n5 3\n3 3\n6 3\n1 1\n138 6\n","output":"Case #1: YES\nCase #2: NO\nCase #3: YES\nCase #4: YES\nCase #5: NO\nCase #6: YES\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"deciding_points_.*input[.]txt"},"output":{"type":"file","fileName":"deciding_points_output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_int();
    let m = input.read_int();
    drop(input);

    if n < m {
        out.print_line((format!("Case #{}:", test_case), false));
        return;
    }
    if n <= 2 * m - 2 {
        out.print_line((format!("Case #{}:", test_case), true));
        return;
    }
    out.print_line((format!("Case #{}:", test_case), n % 2 == 0));
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
    let re = regex::Regex::new("deciding_points_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("deciding_points_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
