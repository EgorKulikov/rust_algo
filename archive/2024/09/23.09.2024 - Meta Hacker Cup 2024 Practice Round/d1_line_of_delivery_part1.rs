//{"name":"D1: Line of Delivery (Part 1)","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/practice-round/problems/D1","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n2 5\n7\n2\n3 1\n9\n5\n7\n4 7\n8\n7\n9\n6\n2 10\n15\n5\n","output":"Case #1: 1 2\nCase #2: 3 4\nCase #3: 3 0\nCase #4: 1 5\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"line_of_delivery_part__.*input[.]txt"},"output":{"type":"file","fileName":"line_of_delivery_part__output.txt","pattern":null},"languages":{"java":{"taskClass":"D1LineOfDeliveryPart1"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        g: usize,
        e: Vec<usize>,
        id: usize,
        dist: Option<usize>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.g = input.read();
            self.e = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            self.e.sort();
            for i in self.e.indices().rev() {
                if self.dist.minim(self.e[i].abs_diff(self.g)) {
                    self.id = self.n - i;
                }
            }
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.id, self.dist));
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
