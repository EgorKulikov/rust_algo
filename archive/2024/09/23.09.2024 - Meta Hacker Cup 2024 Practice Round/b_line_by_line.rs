//{"name":"B: Line by Line","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/practice-round/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n2 50\n3 10\n13 37\n950 95\n","output":"Case #1: 20.710678118654748\nCase #2: 11.544346900318839\nCase #3: 2.940819927087601\nCase #4: 0.005129467915043762\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"line_by_line_.*input[.]txt"},"output":{"type":"file","fileName":"line_by_line_output.txt","pattern":null},"languages":{"java":{"taskClass":"BLineByLine"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::real::Real;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        p: usize,
        ans: Real,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.p = input.read();
        }

        fn solve(&mut self) {
            self.ans = Real(
                ((self.p as f64 / 100.).powf((self.n as f64 - 1.) / (self.n as f64))
                    - self.p as f64 / 100.)
                    * 100.,
            );
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.ans));
        }
    }

    run_parallel::<Job>(input, output, true);
}

pub static TASK_TYPE: TaskType = TaskType::Classic;

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
}
//END MAIN
