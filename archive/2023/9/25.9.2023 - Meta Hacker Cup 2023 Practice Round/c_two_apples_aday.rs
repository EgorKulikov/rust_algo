//{"name":"C: Two Apples a Day","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Practice Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/practice-round/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"7\n3\n6 3 1 2 5\n2\n7 7 7\n1\n1\n3\n1 9 1 1 4\n4\n1 9 1 1 4 9 9\n4\n1 9 10 1 4 6 9\n3\n1000000000 2 10 4 999999994\n","output":"Case #1: 4\nCase #2: 7\nCase #3: 1\nCase #4: -1\nCase #5: 6\nCase #6: -1\nCase #7: 1000000002\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"two_apples_a_day_.*input[.]txt"},"output":{"type":"file","fileName":"two_apples_a_day_output.txt","pattern":null},"languages":{"java":{"taskClass":"CTwoApplesADay"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::out_line;

type PreCalc = ();

fn solve(input: &mut Input, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        a: Vec<i32>,
        ans: Option<i32>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.a = input.read_int_vec(self.n * 2 - 1).sorted();
        }

        fn solve(&mut self) {
            if self.n == 1 {
                self.ans = Some(1);
                return;
            }
            for sum in [
                self.a[0] + self.a[2 * self.n - 2],
                self.a[1] + self.a[2 * self.n - 2],
                self.a[0] + self.a[2 * self.n - 2],
            ] {
                for take_left in [false, true] {
                    let mut skipped = false;
                    let mut left = 0;
                    let mut right = 2 * self.n - 2;
                    let mut res = -1;
                    for _ in 0..self.n {
                        if left < right && self.a[left] + self.a[right] == sum {
                            left += 1;
                            right -= 1;
                            continue;
                        }
                        if skipped {
                            res = -1;
                            break;
                        }
                        if take_left {
                            res = sum - self.a[right];
                            right -= 1;
                        } else {
                            res = sum - self.a[left];
                            left += 1;
                        }
                        skipped = true;
                    }
                    if res > 0 {
                        self.ans.minim(res);
                    }
                }
            }
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
