//{"name":"B: Dim Sum Delivery","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/practice-round/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2 2 1 1\n5 2 3 1\n4 4 3 3\n","output":"Case #1: NO\nCase #2: YES\nCase #3: NO\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"dim_sum_delivery_.*input[.]txt"},"output":{"type":"file","fileName":"dim_sum_delivery_output.txt","pattern":null},"languages":{"java":{"taskClass":"BDimSumDelivery"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        r: i32,
        c: i32,
        ans: bool,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.r = input.read();
            self.c = input.read();
            input.read_int();
            input.read_int();
        }

        fn solve(&mut self) {
            self.ans = self.r > self.c;
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
