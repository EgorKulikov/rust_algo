//{"name":"C: Fall in Line","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/practice-round/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n4\n1 1\n2 2\n-3 -3\n4 4\n4\n1 1\n-1 1\n1 -1\n-1 -1\n7\n4 8\n2 4\n7 2\n6 10\n0 1\n3 4\n4 7\n","output":"Case #1: 0\nCase #2: 2\nCase #3: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"fall_in_line_.*input[.]txt"},"output":{"type":"file","fileName":"fall_in_line_output.txt","pattern":null},"languages":{"java":{"taskClass":"CFallInLine"}}}

use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::random;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        pts: Vec<Point<i64>>,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.pts = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            for _ in 0..100 {
                let i = random().next(self.n);
                let j = (i + random().next_bounds(1, self.n - 1)) % self.n;
                let line = self.pts[i].line(self.pts[j]);
                let mut cnt = 0;
                for &pt in &self.pts {
                    if line.contains(pt) {
                        cnt += 1;
                    }
                }
                if 2 * cnt > self.n {
                    self.ans = self.n - cnt;
                    return;
                }
            }
            self.ans = self.n - 2;
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
