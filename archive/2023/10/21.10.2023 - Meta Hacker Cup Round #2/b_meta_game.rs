//{"name":"B: Meta Game","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-2/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"8\n6\n3 2 3 5 6 4\n4 6 5 3 2 3\n6\n3 3 2 3 5 6\n4 4 6 5 3 2\n6\n4 3 2 3 5 6\n3 4 6 5 3 2\n2\n1 1\n1 1\n4\n2 2 2 2\n1 1 1 1\n5\n3 3 3 3 3\n1 1 1 1 1\n5\n3 3 3 3 3\n1 1 1 1 3\n5\n1 1 1 1 3\n3 3 3 3 3\n","output":"Case #1: 0\nCase #2: 1\nCase #3: -1\nCase #4: -1\nCase #5: 6\nCase #6: -1\nCase #7: 7\nCase #8: 2\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"meta_game_.*input[.]txt"},"output":{"type":"file","fileName":"meta_game_output.txt","pattern":null},"languages":{"java":{"taskClass":"BMetaGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::num_utils::UpperDiv;
use algo_lib::string::hash::{HashBase, SimpleHash, StringHash};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        a: Vec<usize>,
        b: Vec<usize>,
        ans: Option<usize>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read_size();
            self.a = input.read_vec(self.n);
            self.b = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            let all = self
                .a
                .iter()
                .chain(self.b.iter())
                .chain(self.a.iter())
                .chain(self.b.iter())
                .copied()
                .collect::<Vec<_>>();
            let hash = SimpleHash::new(&all);
            let mut rev = all.clone();
            rev.reverse();
            let rev_hash = SimpleHash::new(&rev);
            let mut num_more = vec![0; 3 * self.n + 1];
            num_more[3 * self.n] = 0;
            for i in (0..3 * self.n).rev() {
                if all[i] > all[i + self.n] {
                    num_more[i] = num_more[i + 1] + 1;
                }
            }
            let mut num_less = vec![0; 3 * self.n + 1];
            num_less[3 * self.n] = 0;
            for i in (0..3 * self.n).rev() {
                if all[i] < all[i + self.n] {
                    num_less[i] = num_less[i + 1] + 1;
                }
            }
            for i in 0..2 * self.n {
                let nl = num_less[i];
                let nm = num_more[i + self.n.upper_div(2)];
                let dh = hash.hash(i..i + self.n * 2);
                let rh = rev_hash.hash(2 * self.n - i..4 * self.n - i);
                if nl >= self.n / 2 && nm >= self.n / 2 && dh == rh {
                    self.ans = Some(i);
                    return;
                }
            }
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), self.ans));
        }
    }

    run_parallel::<Job>(input, output);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    HashBase::init();
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
