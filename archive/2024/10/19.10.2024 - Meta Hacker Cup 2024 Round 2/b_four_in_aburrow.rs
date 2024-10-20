//{"name":"B: Four in a Burrow","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-2/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n\nFFFFFFF\nCCCCCCC\nFFFFFFF\nCCCCCCC\nFFFFFFF\nCCCCCCC\n\nFCFCFCF\nFCCFCFC\nCFFCFCF\nCFCFCFC\nCFCFFCF\nCFCFCFC\n\nFCFCFCF\nCCFCFCF\nCFCFCCF\nCFFFCFC\nFCCCCCC\nCFFFFFF\n\nFCFCFCF\nCFCFCFC\nFCFCFCF\nFCFCFCF\nCFCFCFC\nCFCFCFC\n","output":"Case #1: C\nCase #2: ?\nCase #3: F\nCase #4: 0\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"four_in_a_burrow_.*input[.]txt"},"output":{"type":"file","fileName":"four_in_a_burrow_output.txt","pattern":null},"languages":{"java":{"taskClass":"BFourInABurrow"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::HashSet;

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::run_parallel::run_parallel;
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use std::sync::MutexGuard;

type PreCalc = ();

fn solve(mut input: MutexGuard<Input>, out: &mut Output, test_case: usize, _data: &PreCalc) {
    let map = input.read_char_table(6, 7);
    drop(input);

    let mut ans = 0;
    let mut visited = HashSet::new();
    let mut rec = RecursiveFunction3::new(|rec, state: Vec<usize>, turn: usize, mut won: usize| {
        if visited.contains(&(state.clone(), won)) {
            return;
        }
        visited.insert((state.clone(), won));
        if turn == 42 {
            if won > 0 {
                ans.set_bit(won - 1);
            }
            return;
        }
        if won == 0 {
            'outer: for i in 0..6i32 {
                for j in 0..7i32 {
                    for (dx, dy) in [(0, 1), (1, 0), (1, 1), (1, -1)] {
                        let mut good = true;
                        for k in 0..4 {
                            let x = i + dx * k;
                            let y = j + dy * k;
                            if x < 6
                                && y < 7
                                && x >= 0
                                && y >= 0
                                && x as usize >= state[y as usize]
                                && map[(x as usize, y as usize)] == map[(i as usize, j as usize)]
                            {
                                continue;
                            }
                            good = false;
                            break;
                        }
                        if good {
                            if map[(i as usize, j as usize)] == b'C' {
                                won = 1;
                            } else if map[(i as usize, j as usize)] == b'F' {
                                won = 2;
                            }
                            break 'outer;
                        }
                    }
                }
            }
        }
        let who = if turn % 2 == 0 { b'C' } else { b'F' };
        for i in 0..7 {
            if state[i] > 0 && map[(state[i] - 1, i)] == who {
                let mut state = state.clone();
                state[i] -= 1;
                rec.call(state, turn + 1, won);
            }
        }
    });
    rec.call(vec![6; 7], 0, 0);
    out.print_line((
        format!("Case #{}:", test_case),
        match ans {
            0 => "0".to_string(),
            1 => "C".to_string(),
            2 => "F".to_string(),
            3 => "?".to_string(),
            _ => unreachable!(),
        },
    ));
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
