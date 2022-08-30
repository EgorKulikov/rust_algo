//{"name":"A: Second Hands","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n3 2\n1 2 2\n5 3\n1 2 3 3 1\n5 2\n1 2 3 4 5\n5 5\n1 1 2 2 1\n1 1\n1\n","output":"Case #1: YES\nCase #2: YES\nCase #3: NO\nCase #4: NO\nCase #5: YES\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_hands_.*input[.]txt"},"output":{"type":"file","fileName":"second_hands_output.txt","pattern":null},"languages":{"java":{"taskClass":"ASecondHands"}}}

use algo_lib::collections::vec_ext::Qty;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        k: usize,
        s: Vec<usize>,
        ans: bool,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.k = input.read();
            self.s = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            if self.n > self.k * 2 {
                return;
            }
            let q = self.s.qty();
            self.ans = q.into_iter().all(|x| x <= 2);
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
