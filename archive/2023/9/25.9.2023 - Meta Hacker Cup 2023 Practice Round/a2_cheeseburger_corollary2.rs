//{"name":"A2: Cheeseburger Corollary 2","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/practice-round/problems/A2","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n2 3 5\n2 3 2\n2 3 1\n5 1 100\n1 3 100\n1 1 1000000000000\n","output":"Case #1: 3\nCase #2: 1\nCase #3: 0\nCase #4: 199\nCase #5: 100\nCase #6: 1999999999999\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"cheeseburger_corollary__.*input[.]txt"},"output":{"type":"file","fileName":"cheeseburger_corollary__output.txt","pattern":null},"languages":{"java":{"taskClass":"A2CheeseburgerCorollary2"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        a: i64,
        b: i64,
        c: i64,
        ans: i64,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.a = input.read();
            self.b = input.read();
            self.c = input.read();
        }

        fn solve(&mut self) {
            self.a.minim(self.b);
            self.b.minim(2 * self.a);
            if self.c < self.a {
                return;
            }
            self.ans = (self.c - self.a) / self.b * 2 + 1 + (self.c - self.a) % self.b / self.a;
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case), self.ans);
        }
    }

    run_parallel::<Job>(input);
}

pub(crate) fn run(mut input: Input) -> bool {
    let pre_calc = ();
    solve(&mut input, &pre_calc);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    //    stress_test::stress_test(run, tester::check);
}
//END MAIN
