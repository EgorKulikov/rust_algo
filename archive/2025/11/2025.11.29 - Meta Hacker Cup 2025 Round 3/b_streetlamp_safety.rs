//{"name":"B: Streetlamp Safety","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-3/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n3\n3 1 2\n0 1 2\n4\n5 5 5 5\n0 2 3 0\n5\n2 4 1 7 3\n0 0 0 0 0\n6\n5 2 6 1 4 3\n0 2 0 3 1 4\n8\n7 1 8 2 9 3 10 4\n0 1 2 2 3 1 4 5\n","output":"Case #1: 3\nCase #2: 15\nCase #3: 0\nCase #4: 14\nCase #5: 23\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"streetlamp_safety_.*input[.]txt"},"output":{"type":"file","fileName":"streetlamp_safety_output.txt","pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let b = input.read_size_vec(n);
    drop(input);

    // let mut len = Vec::new();
    // let mut pos = Vec::new();
    // let mut last = 0;
    // for i in 0..n {
    //     if b[i] > last {
    //         len.push(b[i]);
    //         pos.push(i);
    //         last = b[i];
    //     }
    // }
    let mut val = Arr2d::new(n + 1, n + 1, i64::MAX / 2);
    val[(0, 0)] = 0;
    for i in 0..n {
        let mut remove = vec![Vec::new(); i + 1];
        let mut set = MultiTreapSet::new();
        for j in b[i]..=i {
            let cand = val[(i, j)];
            val[(i + 1, j)].minim(cand);
        }
        let mut cost = 0;
        for j in (0..=i).rev() {
            cost += a[j];
            set.insert(b[j]);
            if b[j] != 0 {
                remove[j + 1 - b[j]].push(b[j]);
            }
            for k in remove[j].copy_iter() {
                set.remove(&k);
            }
            let need = set.last().copied().unwrap_or(0);
            let len = i - j + 1;
            if len > need {
                let cand = cost + val[(j, need)];
                val[(i + 1, len)].minim(cand);
            }
        }
        for j in (0..i).rev() {
            let cand = val[(i + 1, j + 1)];
            val[(i + 1, j)].minim(cand);
        }
    }

    let ans = val.row(n).min();

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
    let re = regex::Regex::new("streetlamp_safety_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("streetlamp_safety_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
