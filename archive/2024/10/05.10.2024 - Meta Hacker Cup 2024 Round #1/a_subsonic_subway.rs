//{"name":"A: Subsonic Subway","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-1/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n3\n0 10\n0 10\n0 10\n4\n0 3\n1 4\n1 2\n0 10\n2\n5 10\n0 5\n2\n1 2\n4 10\n","output":"Case #1: 0.3\nCase #2: 1.5\nCase #3: -1\nCase #4: 0.5\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"subsonic_subway_.*input[.]txt"},"output":{"type":"file","fileName":"subsonic_subway_output.txt","pattern":null},"languages":{"java":{"taskClass":"ASubsonicSubway"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::option::OptionExt;
use algo_lib::misc::test_type::{TaskType, TestType};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::invertible::Invertible;
use algo_lib::numbers::rational::Rational;
use algo_lib::numbers::real::Real;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        stations: Vec<(i64, i64)>,
        ans: Option<Real>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.stations = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            let mut min_speed = Rational::zero();
            let mut max_speed = Rational::new(1_000_000, 1);
            for (i, &(a, b)) in self.stations.iter().enumerate() {
                min_speed.maxim(Rational::new(a, i as i64 + 1));
                max_speed.minim(Rational::new(b, i as i64 + 1));
            }
            self.ans = max_speed
                .real()
                .inv()
                .unwrap()
                .take_if(min_speed <= max_speed);
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
