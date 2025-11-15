//{"name":"E: Descending Platforms","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-2/problems/E","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n3 27\n3 5 1\n3 18\n1 9 6\n3 28\n3 5 1\n5 100\n7 6 16 9 3\n10 45\n1 2 4 8 1 2 60 3 5 7\n8 230\n5 11 12 15 13 7 23 7\n","output":"Case #1: 7\n4 3 0\nCase #2: 4\n2 2 0\nCase #3: 8\n4 4 0\nCase #4: 11\n5 3 3 0 0\nCase #5: 7\n1 1 1 1 1 1 1 0 0 0\nCase #6: 20\n3 3 3 3 3 3 2 0\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"descending_platforms_.*input[.]txt"},"output":{"type":"file","fileName":"descending_platforms_output.txt","pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::numbers::rational::Rational;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_long();
    let a = input.read_long_vec(n);
    drop(input);

    let mut vars = vec![0];
    let mut sum = 0;
    let mut best_ratio = Rational::zero();
    let mut best_at = 0;
    for i in 0..n {
        sum += a[i];
        vars.push(sum);
        if best_ratio.maxim(Rational::new(sum, i as i64 + 1)) {
            best_at = i + 1;
        }
    }

    let mut ans = m.upper_div(vars[best_at]) * best_at as i64;
    let mut qty = vec![0; n];
    let mut res = vec![i64::MIN / 2; best_at];
    res[0] = 0;
    let mut last = vec![n + 1; best_at];

    for i in 1..=n {
        let g = gcd(i, best_at);
        for j in 0..g {
            let mut at = j;
            let mut cur = res[at];
            for _ in 0..2 * best_at / g {
                if res[at].maxim(cur) {
                    last[at] = i;
                }
                cur.maxim(res[at]);
                at += i;
                cur += vars[i];
                let times = at / best_at;
                at %= best_at;
                cur -= times as i64 * vars[best_at];
            }
        }
    }
    let add = m / vars[best_at] * vars[best_at];
    let base = ans - best_at as i64;
    let mut best = 0;
    for i in 0..best_at {
        if res[i] + add >= m {
            if ans.minim(i as i64 + base) {
                best = i;
            }
        }
    }
    let mut sum = 0;
    if best == 0 {
        qty[best_at - 1] = m.upper_div(vars[best_at]);
        sum = qty[best_at - 1] * vars[best_at];
    } else {
        let mut cur = best;
        while cur != 0 {
            qty[last[cur] - 1] += 1;
            let l = last[cur];
            cur += best_at;
            cur -= l % best_at;
            sum += vars[l];
            cur %= best_at;
        }
    }
    if sum >= m + vars[best_at] {
        qty.fill(0);
        let mut res = vec![0; n];
        let mut last = vec![n + 1; n];
        for i in 0.. {
            if res[i] >= m {
                ans = i as i64;
                break;
            }
            res.push(0);
            last.push(0);
            for j in 1..=n {
                let cand = res[i] + vars[j];
                if res[i + j].maxim(cand) {
                    last[i + j] = j;
                }
            }
        }
        let mut cur = ans as usize;
        while cur > 0 {
            qty[last[cur] - 1] += 1;
            cur -= last[cur];
        }
    } else {
        qty[best_at - 1] += (m - sum).upper_div(vars[best_at]);
    }
    for i in (0..n - 1).rev() {
        qty[i] += qty[i + 1];
    }

    out.print_line((format!("Case #{}:", test_case), ans));
    out.print_line(qty);
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
    let re = regex::Regex::new("descending_platforms_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("descending_platforms_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
