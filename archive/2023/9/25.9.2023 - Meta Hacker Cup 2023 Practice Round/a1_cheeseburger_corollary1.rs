//{"name":"A1: Cheeseburger Corollary 1","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/practice-round/problems/A1","interactive":false,"timeLimit":360000,"tests":[{"input":"7\n1 1 3\n0 2 4\n5 5 1\n0 1 1\n1 1 2\n97 1 99\n97 1 100\n","output":"Case #1: YES\nCase #2: NO\nCase #3: YES\nCase #4: YES\nCase #5: YES\nCase #6: YES\nCase #7: NO\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"cheeseburger_corollary__.*input[.]txt"},"output":{"type":"file","fileName":"cheeseburger_corollary__output.txt","pattern":null},"languages":{"java":{"taskClass":"A1CheeseburgerCorollary1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        s: i32,
        d: i32,
        k: i32,
        ans: bool,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.s = input.read();
            self.d = input.read();
            self.k = input.read();
        }

        fn solve(&mut self) {
            let buns = (self.s + self.d) * 2;
            let meat = self.s + self.d * 2;
            self.ans = self.k + 1 <= buns && self.k <= meat;
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
