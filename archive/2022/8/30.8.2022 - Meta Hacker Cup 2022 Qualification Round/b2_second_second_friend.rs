//{"name":"B2: Second Second Friend","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/B2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n1 3\n.^.\n3 1\n.\n#\n#\n4 4\n..^.\n.#^#\n....\n...^\n","output":"Case #1: Impossible\nCase #2: Possible\n.\n#\n#\nCase #3: Possible\n^^^.\n^#^#\n^^^^\n..^^\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_second_friend_.*input[.]txt"},"output":{"type":"file","fileName":"second_second_friend_output.txt","pattern":null},"languages":{"java":{"taskClass":"B2SecondSecondFriend"}}}

use algo_lib::collections::arr2d::{Arr2d, Arr2dCharWrite, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::{output, set_bool_output, BoolOutput};
use algo_lib::misc::dirs::D4;
use std::collections::VecDeque;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    set_bool_output(BoolOutput::PossibleImpossible);
    #[derive(Clone, Default)]
    struct Job {
        r: usize,
        c: usize,
        grid: Arr2d<char>,
        ans: bool,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.r = input.read();
            self.c = input.read();
            self.grid = input.read_table(self.r, self.c);
        }

        fn solve(&mut self) {
            let mut bad = Arr2d::new(self.r, self.c, false);
            let mut q = VecDeque::new();
            for i in 0..self.r {
                for j in 0..self.c {
                    if self.grid[(i, j)] == '#' {
                        bad[(i, j)] = true;
                        q.push_back((i, j));
                    }
                }
            }
            if self.r == 1 || self.c == 1 {
                bad[(0, 0)] = true;
                q.push_back((0, 0));
            }
            while let Some((r, c)) = q.pop_front() {
                for (nr, nc) in D4::iter(r, c, self.r, self.c) {
                    if !bad[(nr, nc)] {
                        let mut qty = 0;
                        for (or, oc) in D4::iter(nr, nc, self.r, self.c) {
                            if !bad[(or, oc)] {
                                qty += 1;
                            }
                        }
                        if qty <= 1 {
                            bad[(nr, nc)] = true;
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
            for i in 0..self.r {
                for j in 0..self.c {
                    if bad[(i, j)] && self.grid[(i, j)] == '^' {
                        return;
                    }
                    if !bad[(i, j)] {
                        self.grid[(i, j)] = '^';
                    }
                }
            }
            self.ans = true;
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.ans);
            if self.ans {
                output().print_table(&self.grid);
            }
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
