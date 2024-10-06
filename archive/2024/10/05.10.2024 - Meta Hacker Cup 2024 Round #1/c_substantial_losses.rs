//{"name":"C: Substantial Losses","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-1/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n201 200 1\n185 183 2\n250 180 0\n77665544332211 11223344556677 0\n83716485936440 64528193749358 1938563682\n","output":"Case #1: 3\nCase #2: 10\nCase #3: 70\nCase #4: 53884207\nCase #5: 306870714\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"substantial_losses_.*input[.]txt"},"output":{"type":"file","fileName":"substantial_losses_output.txt","pattern":null},"languages":{"java":{"taskClass":"CSubstantialLosses"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::mod_int::ModIntF;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    type Mod = ModIntF;

    #[derive(Clone, Default)]
    struct Job {
        w: i64,
        g: i64,
        l: i64,
        ans: Mod,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.w = input.read();
            self.g = input.read();
            self.l = input.read();
        }

        fn solve(&mut self) {
            self.ans = Mod::new_from_wide(self.w - self.g) * Mod::new_from_wide(2 * self.l + 1);
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
