//{"name":"C: Monkey Around","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/practice-round/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n7\n1 2 3 3 4 1 2\n5\n1 2 3 4 5\n4\n3 1 2 1\n4\n2 1 2 1\n","output":"Case #1: 5\n1 3\n2\n1 4\n2\n2\nCase #2: 1\n1 5\nCase #3: 4\n1 3\n2\n2\n1 1\nCase #4: 3\n1 2\n1 2\n2\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"monkey_around_.*input[.]txt"},"output":{"type":"file","fileName":"monkey_around_output.txt","pattern":null}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();
    drop(input);

    let mut start = 0;
    let mut len = Vec::new();
    let mut shift = vec![a[0]];
    let mut has_zero = a[0] == 0;
    for i in 1..n {
        let x = a[i - 1];
        let y = a[i];
        if (x + 1 != y && (has_zero || y != 0)) || y == shift[Back(0)] {
            len.push(i - start);
            start = i;
            has_zero = false;
            shift.push(y);
        }
        has_zero |= y == 0;
    }
    len.push(n - start);
    let mut num_shift = 0;
    let mut ans = Vec::new();
    for (l, s) in len.iter_zip(shift).rev() {
        let mut cur = num_shift % l;
        while cur != s {
            ans.push(vec![2]);
            cur = (cur + 1) % l;
            num_shift += 1;
        }
        ans.push(vec![1, l]);
    }
    ans.reverse();
    out.print_line((format!("Case #{}:", test_case), ans.len()));
    out.print_per_line(&ans);
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
    let re = regex::Regex::new("monkey_around_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("monkey_around_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
