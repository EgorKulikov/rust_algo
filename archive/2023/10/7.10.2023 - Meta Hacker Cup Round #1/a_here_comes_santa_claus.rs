//{"name":"A: Here Comes Santa Claus","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-1/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n7\n1 17 3 13 7 23 12\n5\n5 4 3 2 1\n4\n10 100 1000 10000\n","output":"Case #1: 18\nCase #2: 2.5\nCase #3: 5445\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"here_comes_santa_claus_.*input[.]txt"},"output":{"type":"file","fileName":"here_comes_santa_claus_output.txt","pattern":null},"languages":{"java":{"taskClass":"AHereComesSantaClaus"}}}

use algo_lib::collections::slice_ext::reversed::ReversedSlice;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::num_traits::real::IntoReal;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        x: Vec<i64>,
        ans: f64,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.x = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            self.x.sort();
            if self.n != 5 {
                self.ans = (self.x.rev()[0] + self.x.rev()[1] - self.x[0] - self.x[1]) as f64 / 2.0;
                return;
            }
            let c1 = self.x.rev()[0] + self.x.rev()[1] - self.x[0] - self.x[2];
            let c2 = self.x.rev()[0] + self.x.rev()[2] - self.x[0] - self.x[1];
            self.ans = c1.max(c2) as f64 / 2.0;
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.ans.into_real()));
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
