//{"name":"E: Wildcard Submissions","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-1/problems/E","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n2\nMETA\nMATE\n2\n?B\nAC\n1\n??\n3\nXXY\nX?\n?X\n2\n??M?E?T?A??\n?M?E?T?A?\n","output":"Case #1: 8\nCase #2: 54\nCase #3: 703\nCase #4: 79\nCase #5: 392316013\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"wildcard_submissions_.*input[.]txt"},"output":{"type":"file","fileName":"wildcard_submissions_output.txt","pattern":null},"languages":{"java":{"taskClass":"EWildcardSubmissions"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable4, RecursiveFunction4};
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::misc::test_type::TaskType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::string::str::{Str, StrReader};
use std::sync::MutexGuard;

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize) {
    let n = input.read_size();
    let s = input.read_str_vec(n);
    drop(input);
    type Mod = ModIntF;
    let mut ans = Mod::zero();

    let mut rec = RecursiveFunction4::new(
        |rec, pos: usize, mut state: Str<'static>, coef: Mod, any: bool| {
            if pos == n {
                if any {
                    let mut cur = Mod::one();
                    let mut add = Mod::one();
                    for i in 0..state.len() {
                        if state[i] == b'?' {
                            cur *= Mod::new(26);
                        }
                        add += cur;
                    }
                    ans += coef * add;
                }
                return;
            }
            rec.call(pos + 1, state.clone(), coef, any);
            for i in 0..state.len() {
                if s[pos].len() == i
                    || s[pos][i] != b'?' && state[i] != b'?' && s[pos][i] != state[i]
                {
                    state.resize(i, b'?');
                    rec.call(pos + 1, state, -coef, true);
                    return;
                }
                if s[pos][i] != b'?' {
                    state[i] = s[pos][i];
                }
            }
            rec.call(pos + 1, state, coef, any);
        },
    );
    rec.call(0, Str::from(vec![b'?'; 101]), -Mod::one(), false);

    out.print_line((format!("Case #{}:", test_case), ans));
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(input: Input, mut output: Output) -> bool {
    let is_exhausted = run_parallel(input, &mut output, true, solve);
    output.flush();
    is_exhausted
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
