//{"name":"A: Walk the Line","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/practice-round/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n4 17\n1\n2\n5\n10\n4 4\n1\n2\n5\n10\n2 22\n22\n22\n3 1000000000\n1000000000\n1000000000\n1000000000\n1 10\n12\n1 100\n12\n","output":"Case #1: YES\nCase #2: NO\nCase #3: YES\nCase #4: NO\nCase #5: NO\nCase #6: YES\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"walk_the_line_.*input[.]txt"},"output":{"type":"file","fileName":"walk_the_line_output.txt","pattern":null},"languages":{"java":{"taskClass":"AWalkTheLine"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
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
            let min = *self.s.iter().min().unwrap();
            if self.n == 1 {
                self.ans = self.k >= min;
            } else {
                self.ans = self.k >= min * (2 * self.n - 3);
            }
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
