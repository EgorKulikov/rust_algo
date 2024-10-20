//{"name":"A1: Cottontail Climb (Part 1)","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-2/problems/A1","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n121 121 11\n0 100 2\n0 132 1\n121 132 1\n121 131 1\n22322 22322 1\n","output":"Case #1: 1\nCase #2: 4\nCase #3: 10\nCase #4: 1\nCase #5: 1\nCase #6: 0\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"cottontail_climb_part_1_.*input[.]txt"},"output":{"type":"file","fileName":"cottontail_climb_part__output.txt","pattern":null},"languages":{"java":{"taskClass":"A1CottontailClimbPart1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::run_parallel::run_parallel;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let m = input.read_long();
    drop(input);

    let mut ans = 0;
    for i in 1..10 {
        for j in i..10 {
            let mut cur = 0i64;
            for k in i..=j {
                cur *= 10;
                cur += k;
            }
            for k in (i..j).rev() {
                cur *= 10;
                cur += k;
            }
            if cur >= a && cur <= b && cur % m == 0 {
                ans += 1;
            }
        }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
