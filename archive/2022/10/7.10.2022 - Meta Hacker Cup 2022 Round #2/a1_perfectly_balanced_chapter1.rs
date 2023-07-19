//{"name":"A1: Perfectly Balanced - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/A1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\nsingingbanana\n5\n8 12\n9 13\n8 10\n10 12\n1 7\nprepareintelligentopinion\n4\n1 7\n8 18\n19 25\n12 13\nphpservers\n6\n1 3\n4 10\n1 3\n2 2\n3 5\n1 10\n","output":"Case #1: 4\nCase #2: 3\nCase #3: 4\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"perfectly_balanced_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"perfectly_balanced_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"A1PerfectlyBalancedChapter1"}}}

use algo_lib::collections::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        s: Str<'static>,
        q: usize,
        queries: Vec<(usize, usize)>,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.s = input.read();
            self.q = input.read();
            self.queries = input.read_vec(self.q);
        }

        fn solve(&mut self) {
            let mut q = Arr2d::new(self.s.len() + 1, 26, 0);
            for (i, c) in self.s.iter().enumerate() {
                for j in 0..26 {
                    q[(i + 1, j)] = q[(i, j)];
                }
                q[(i + 1, c as usize - 'a' as usize)] += 1;
            }

            for &(l, r) in &self.queries {
                if (r - l) % 2 != 0 {
                    continue;
                }
                let m = (l + r) / 2;
                let mut bad = 0;
                for j in 0..26 {
                    if q[(m, j)] - q[(l - 1, j)] != q[(r, j)] - q[(m, j)] {
                        bad += 1;
                    }
                }
                if bad == 1 {
                    self.ans += 1;
                    continue;
                }
                let mut bad = 0;
                for j in 0..26 {
                    if q[(m - 1, j)] - q[(l - 1, j)] != q[(r, j)] - q[(m - 1, j)] {
                        bad += 1;
                    }
                }
                if bad == 1 {
                    self.ans += 1;
                    continue;
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
