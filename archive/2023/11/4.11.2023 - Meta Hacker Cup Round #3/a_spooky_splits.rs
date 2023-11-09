//{"name":"A: Spooky Splits","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-3/problems/A?source=facebook","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n12 7\n1 2\n2 4\n4 6\n3 8\n8 12\n9 10\n10 11\n3 0\n8 4\n1 7\n2 5\n6 3\n8 4\n","output":"Case #1: 1 2 3\nCase #2: 1 3\nCase #3: 1 2 4\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"spooky_splits_.*input[.]txt"},"output":{"type":"file","fileName":"spooky_splits_output.txt","pattern":null},"languages":{"java":{"taskClass":"ASpookySplits"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::memo::memoization::Memoization2;
use algo_lib::misc::recursive_function::Callable2;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        m: usize,
        pairs: Vec<(usize, usize)>,
        ans: Vec<usize>,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.m = input.read();
            self.pairs = input.read_vec(self.m).dec();
        }

        fn solve(&mut self) {
            let mut dsu = DSU::new(self.n);
            for (a, b) in &self.pairs {
                dsu.join(*a, *b);
            }
            let mut parts = Vec::new();
            for i in dsu.iter() {
                parts.push(dsu.size(i));
            }
            parts.sort();
            parts.reverse();
            for k in 1..=self.n {
                if self.n % k != 0 {
                    continue;
                }
                let mut mem = Memoization2::new(|f, step: usize, v: Vec<usize>| {
                    if step == parts.len() {
                        return true;
                    }
                    for i in v.indices() {
                        if v[i] >= parts[step] {
                            let mut nv = v.clone();
                            nv[i] -= parts[step];
                            nv.sort();
                            if f.call(step + 1, nv) {
                                return true;
                            }
                        }
                    }
                    false
                });
                if mem.call(0, vec![self.n / k; k]) {
                    self.ans.push(k);
                }
            }
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), &self.ans));
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
