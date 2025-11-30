//{"name":"C: Adversarial Attack","group":"Meta Coding Competitions - Meta Hacker Cup 2025 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2025/round-3/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n3 9\nsand\nandy\nmeta\n2 9\nbanana\nanana\n1 5\napple\n2 10\n1a2a3a\naab\n2 8\nbora\nbora\n2 10\ntournament\ntour\n","output":"Case #1: 9\nCase #2: 14\nCase #3: 5\nCase #4: 24\nCase #5: 12\nCase #6: 0\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"adversarial_attack_.*input[.]txt"},"output":{"type":"file","fileName":"adversarial_attack_output.txt","pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::time_tracker::TimeTracker;
use algo_lib::string::hash::{SimpleHash, StringHash};
use algo_lib::string::str::{Str, StrReader};
use std::mem::swap;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let ww = input.read_str_vec(n);
    drop(input);

    let mut tt = TimeTracker::new();
    let w = Vec::with_gen(n, |i| {
        let mut cur = usize::MAX;
        let mut res = Str::new();
        for c in ww[i].copy_iter() {
            if c.is_ascii_digit() {
                if cur == usize::MAX {
                    cur = 0;
                }
                cur *= 10;
                cur += c as usize - b'0' as usize;
            } else {
                if cur == usize::MAX {
                    cur = 1;
                }
                for _ in 0..cur {
                    res.push(c);
                }
                cur = usize::MAX;
            }
        }
        res
    });

    tt.milestone("build");
    let mut min_len = w[0].len();
    let mut lh = SimpleHash::new(&w[0]);
    let mut len = 0;
    for i in w[0].indices() {
        if w[0][Back(i)] == w[0][Back(0)] {
            len += 1;
        } else {
            break;
        }
    }
    let mut shifts = Vec::new();
    for i in 1..n {
        let ch = SimpleHash::new(&w[i]);
        let mut cur = Vec::new();
        let mut base = 0;
        let mut first = true;
        let mut tail = 0;
        if w[i][0] == w[i - 1][Back(0)] {
            let mut x = 0;
            for j in w[i].indices() {
                if w[i][j] == w[i][0] {
                    x += 1;
                } else {
                    break;
                }
            }
            tail = x.min(len);
            tail -= 1;
        }
        for j in (tail..=w[i - 1].len().min(w[i].len())).rev() {
            if lh.hash(w[i - 1].len() - j..) == ch.hash(..j) {
                if first {
                    base = w[i].len() - j;
                    min_len += w[i].len() - j;
                    first = false;
                } else {
                    cur.push(w[i].len() - j - base);
                }
            }
        }
        if !cur.is_empty() {
            shifts.push((cur, w[i].len() - base - tail, w[i].len() - base));
        }
        lh = ch;
        len = 0;
        for j in w[i].indices() {
            if w[i][Back(j)] == w[i][Back(0)] {
                len += 1;
            } else {
                break;
            }
        }
    }
    tt.milestone("shifts");
    if min_len > k {
        out.print_line((format!("Case #{}:", test_case), 0));
        return;
    }
    let mut cur = BitSet::new(k - min_len + 1);
    cur.set(0);
    let mut next = BitSet::new(k - min_len + 1);
    let mut c = 0;
    for (sh, mut tail_start, tail_finish) in shifts {
        c += 1;
        let mut last = sh[Back(0)];
        for i in sh.copy_iter().rev() {
            next <<= last - i;
            next |= &cur;
            last = i;
        }
        next <<= sh[0];
        next |= &cur;
        if tail_start != tail_finish {
            cur <<= tail_start;
            let mut len = 1;
            while tail_finish - tail_start + 1 >= len * 2 {
                cur.shift_left_or(len);
                len *= 2;
            }
            cur.shift_left_or(tail_finish - tail_start + 1 - len);
            next |= &cur;
        }
        swap(&mut cur, &mut next);
    }
    let mut ans = 0;
    for i in 0..=k - min_len {
        if cur[i] {
            ans += i + min_len;
        }
    }
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
    let re = regex::Regex::new("adversarial_attack_.*input[.]txt").unwrap();
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
    let out_file = std::fs::File::create("adversarial_attack_output.txt").unwrap();
    let output = algo_lib::io::output::Output::file(out_file);
    run(input, output);
}
