//{"name":"A2: Cottontail Climb (Part 2)","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-2/problems/A2","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n121 121 11\n0 100 2\n0 132 1\n121 132 1\n121 131 1\n22322 22322 1\n","output":"Case #1: 1\nCase #2: 4\nCase #3: 12\nCase #4: 3\nCase #5: 2\nCase #6: 1\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"cottontail_climb_part_2_.*input[.]txt"},"output":{"type":"file","fileName":"cottontail_climb_part_2_output.txt","pattern":null},"languages":{"java":{"taskClass":"A2CottontailClimbPart2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::HashMap;

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::numbers::num_utils::powers;
use algo_lib::numbers::number_ext::NumDigs;
use algo_lib::numbers::number_iterator::iterate;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let m = input.read_long();
    drop(input);

    let mut map = HashMap::<(usize, usize, i64), DefaultHashMap<i64, usize>>::new();
    let tens = powers(10i64, 19);
    for k in 0..=8 {
        for j in 0..=2 * k {
            for i in 1..=9 {
                let mut cur = DefaultHashMap::<i64, usize>::new();
                if j == 0 {
                    cur[0] += 1;
                } else {
                    let seg = if j < k {
                        1..=i
                    } else if j == k {
                        1..=i - 1
                    } else if j == k + 1 {
                        i + 1..=9
                    } else {
                        i..=9
                    };
                    for l in seg {
                        for (mod_val, cnt) in map[&(k, j - 1, l)].iter() {
                            cur[(*mod_val + l * tens[j - 1]) % m] += *cnt;
                        }
                    }
                }
                map.insert((k, j, i), cur);
            }
        }
    }
    let mut ans = 0;
    for (prefix, len, _) in iterate(a, b) {
        let digs = prefix.num_digs();
        let total_len = len + digs;
        if total_len % 2 == 0 {
            continue;
        }
        let k = total_len / 2;
        let mut ok = true;
        for i in 1..digs {
            let d1 = prefix / tens[digs - i] % 10;
            let d2 = prefix / tens[digs - i - 1] % 10;
            if d1 == 0 || d2 == 0 {
                ok = false;
                break;
            }
            if i < k && d1 > d2 {
                ok = false;
                break;
            }
            if i == k && d1 >= d2 {
                ok = false;
                break;
            }
            if i == k + 1 && d1 <= d2 {
                ok = false;
                break;
            }
            if i > k && d1 < d2 {
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }
        ans += map[&(k, len, prefix % 10)][(m - prefix * tens[len] % m) % m];
    }
    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let is_exhausted = run_parallel(input, &mut output, false, (), solve);
    output.flush();
    is_exhausted
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
