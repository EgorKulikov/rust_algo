//{"name":"C2: Second Second Meaning","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/C2","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n3\n.-.\n4\n-\n3\n..\n","output":"Case #1:\n...\n---\nCase #2:\n...\n.-\n..-\nCase #3:\n-\n.-\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_second_meaning_.*input[.]txt"},"output":{"type":"file","fileName":"second_second_meaning_output.txt","pattern":null},"languages":{"java":{"taskClass":"C2SecondSecondMeaning"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::num_traits::bit_ops::BitOps;
use algo_lib::out_line;
use algo_lib::string::string::Str;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        s: Str<'static>,
        ans: Vec<Str<'static>>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.s = input.read();
        }

        fn solve(&mut self) {
            let first = b'.' + b'-' - self.s[0];
            for i in 0..self.n - 1 {
                let mut cur = Vec::with_capacity(10);
                cur.push(first);
                for j in 0..9 {
                    if i.is_set(j) {
                        cur.push(b'.');
                    } else {
                        cur.push(b'-');
                    }
                }
                self.ans.push(cur.into());
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}:", test_case));
            output().print_per_line(&self.ans);
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
