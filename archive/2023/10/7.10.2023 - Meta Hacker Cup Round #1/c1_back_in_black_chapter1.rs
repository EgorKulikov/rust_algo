//{"name":"C1: Back in Black (Chapter 1)","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-1/problems/C1","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n4\n1010\n1\n1\n4\n0001\n4\n2\n3\n2\n4\n7\n0101101\n8\n1\n3\n2\n6\n7\n4\n2\n5\n7\n0101100\n1\n7\n7\n1111111\n1\n1\n","output":"Case #1: 1\nCase #2: 1\nCase #3: 4\nCase #4: 4\nCase #5: 0\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"back_in_black_chapter_._.*input[.]txt"},"output":{"type":"file","fileName":"back_in_black_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"C1BackInBlackChapter1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashSet;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::string::str::Str;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        s: Str<'static>,
        q: usize,
        b: Vec<usize>,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.s = input.read();
            self.q = input.read();
            self.b = input.read_vec(self.q);
        }

        fn solve(&mut self) {
            let mut moves = HashSet::new();
            for i in 1..=self.n {
                if self.s[i - 1] == b'1' {
                    moves.insert(i);
                    for j in (i..=self.n).step_by(i) {
                        self.s[j - 1] = b'1' + b'0' - self.s[j - 1];
                    }
                }
            }
            for &q in &self.b {
                if moves.contains(&q) {
                    moves.remove(&q);
                } else {
                    moves.insert(q);
                }
                self.ans += moves.len();
            }
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.ans));
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
