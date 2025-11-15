//{"name":"B: Defining Prizes","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-2/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n3 2\n1 2 3\n1 2\n3 1\n1 1 1\n2\n3 1\n1 1 1\n3\n5 5\n5 4 3 2 1\n1 2 3 4 5\n5 4\n0 0 1 1 2\n1 1 1 1\n","output":"Case #1: 2\nCase #2: 0\nCase #3: 3\nCase #4: 5\nCase #5: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"defining_prizes_.*input[.]txt"},"output":{"type":"file","fileName":"defining_prizes_output.txt","pattern":null}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n).sorted();
    let b = input.read_size_vec(m).sorted();
    drop(input);

    let mut qty = Vec::new();
    let mut start = 0;
    for i in 1..n {
        if a[i] != a[i - 1] {
            qty.push(i - start);
            start = i;
        }
    }
    qty.push(n - start);
    let mut left = 0;
    let mut right = m.min(qty.len());
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut free = 0;
        let mut need = 0;
        let mut good = true;
        for i in 0..m {
            if m - i <= mid {
                need += qty[Back(mid - (m - i))];
            }
            if b[i] >= need {
                free += b[i] - need;
            } else {
                if free + b[i] >= need {
                    free -= need - b[i];
                } else {
                    good = false;
                    break;
                }
            }
        }
        if good {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    let mut ans = 0;
    for i in 0..left {
        ans += qty[Back(i)];
    }

    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let pre_calc = ();
    let is_exhausted = run_parallel(input, &mut output, false, pre_calc, solve);
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
    let re = regex::Regex::new("defining_prizes_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("defining_prizes_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
