//{"name":"B1: Watering Well - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-1/problems/B1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2\n2 2\n5 5\n2\n2 5\n6 6\n4\n1 1\n4 3\n6 3\n6 5\n3\n3 1\n5 2\n6 5\n8\n2837 745\n62 1162\n2634 1112\n1746 2618\n847 127\n986 1993\n732 1273\n2003 1998\n4\n1276 2231\n1234 1234\n287 2371\n3000 3000\n","output":"Case #1: 52\nCase #2: 131\nCase #3: 110090622\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"watering_well_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"watering_well_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"B1WateringWellChapter1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::as_index::AsIndex;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::out_line;

fn solve(input: &mut Input) {
    type Mod = ModInt7;

    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        pts: Vec<(Mod, Mod)>,
        q: usize,
        check: Vec<(Mod, Mod)>,
        ans: Mod,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.pts = input.read_vec(self.n);
            self.q = input.read();
            self.check = input.read_vec(self.q);
        }

        fn solve(&mut self) {
            let mut sum_x = Mod::zero();
            let mut sum_y = Mod::zero();
            let mut sum_x2 = Mod::zero();
            let mut sum_y2 = Mod::zero();
            for &(x, y) in &self.pts {
                sum_x += x;
                sum_y += y;
                sum_x2 += x * x;
                sum_y2 += y * y;
            }
            for &(x, y) in &self.check {
                self.ans += Mod::from_index(self.n) * (x * x + y * y) + (sum_x2 + sum_y2)
                    - (sum_x * x + sum_y * y) * Mod::new(2);
            }
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
