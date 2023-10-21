//{"name":"A1: Ready, Go (Part 1)","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-2/problems/A1","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n4 4\n.B..\nBW..\n.B..\n....\n4 5\nWWWB.\nWB...\nWB.WB\nB.W..\n4 5\nWWWB.\nWB...\nWB.WB\n..W..\n3 5\n..W..\n.WBWW\n.B..W\n3 3\n.WB\nWWB\nBB.\n","output":"Case #1: YES\nCase #2: YES\nCase #3: NO\nCase #4: NO\nCase #5: YES\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"ready_go_part_._.*input[.]txt"},"output":{"type":"file","fileName":"ready_go_part__output.txt","pattern":null},"languages":{"java":{"taskClass":"A1ReadyGoPart1"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use std::collections::{HashSet, VecDeque};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        r: usize,
        c: usize,
        a: Arr2d<char>,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.r = input.read_size();
            self.c = input.read_size();
            self.a = input.read_char_table(self.r, self.c);
        }

        fn solve(&mut self) {
            let mut captured = DefaultHashMap::<_, usize>::new();
            let mut done = Arr2d::new(self.r, self.c, false);
            for i1 in 0..self.r {
                for j1 in 0..self.c {
                    if done[(i1, j1)] || self.a[(i1, j1)] != 'W' {
                        continue;
                    }
                    let mut queue = VecDeque::new();
                    queue.push_back((i1, j1));
                    let mut free = HashSet::new();
                    let mut qty = 0;
                    done[(i1, j1)] = true;
                    while let Some((i2, j2)) = queue.pop_front() {
                        qty += 1;
                        for (x, y) in D4::iter(i2, j2, self.r, self.c) {
                            if done[(x, y)] {
                                continue;
                            }
                            if self.a[(x, y)] == '.' {
                                free.insert((x, y));
                            } else if self.a[(x, y)] == 'W' {
                                queue.push_back((x, y));
                                done[(x, y)] = true;
                            }
                        }
                    }
                    if free.len() == 1 {
                        captured[free.into_iter().next().unwrap()] += qty;
                    }
                }
            }
            self.ans = captured.values().max().unwrap_or(&0).clone();
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.ans));
        }
    }

    run_parallel::<Job>(input, output);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    solve(&mut input, &mut output, &pre_calc);
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
