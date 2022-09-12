//{"name":"A1: Consecutive Cuts - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-1/problems/A1","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n5 1\n5 1 2 4 3\n2 4 3 5 1\n4 10\n3 1 4 2\n1 2 3 4\n4 0\n3 1 4 2\n2 3 1 4\n3 3\n3 2 1\n1 3 2\n","output":"Case #1: YES\nCase #2: NO\nCase #3: NO\nCase #4: YES\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"consecutive_cuts_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"consecutive_cuts_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"A1ConsecutiveCutsChapter1"}}}

use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::vec_ext::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use std::ops::Deref;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;
use algo_lib::string::string_algorithms::StringAlgorithms;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        k: usize,
        a: Vec<usize>,
        b: Vec<usize>,
        ans: bool,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.k = input.read();
            self.a = input.read_vec(self.n).dec_by_one();
            self.b = input.read_vec(self.n).dec_by_one();
        }

        fn solve(&mut self) {
            let c = self.a.iter().chain(self.b.iter()).cloned().collect_vec();
            let zd = c.deref().z_algorithm();
            let d = self.b.iter().chain(self.a.iter()).cloned().collect_vec();
            let zr = d.deref().z_algorithm();

            for i in 0..self.n {
                if zd[i + self.n] == self.n - i && (i == 0 || zr[2 * self.n - i] == i) {
                    if self.n == 2 {
                        if self.k % 2 == i % 2 {
                            self.ans = true;
                            return;
                        }
                    } else {
                        if self.k == 0 {
                            if i == 0 {
                                self.ans = true;
                                return;
                            }
                        } else if self.k == 1 {
                            if i != 0 {
                                self.ans = true;
                                return;
                            }
                        } else {
                            self.ans = true;
                            return;
                        }
                    }
                }
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.ans);
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
