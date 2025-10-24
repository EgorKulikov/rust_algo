//{"name":"C: Narrowing Down","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-1/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n2\n0 0\n3\n1 1 1\n3\n1 2 3\n7\n0 1 0 2 0 3 0\n","output":"Case #1: 0\nCase #2: 8\nCase #3: 9\nCase #4: 72\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"narrowing_down_.*input[.]txt"},"output":{"type":"file","fileName":"narrowing_down_output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n);
    drop(input);

    let mut ans = 0;
    let mut qty = DefaultHashMap::new(0usize);
    qty[0] += 1;
    let mut xor = 0;
    for i in 0..n {
        xor ^= a[i];
        ans += (i + 1) * (i + 2) / 2;
        let cur = qty[xor];
        ans -= cur * (cur + 1) / 2;
        qty[xor] += 1;
    }
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
    let re = regex::Regex::new("narrowing_down_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("narrowing_down_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
