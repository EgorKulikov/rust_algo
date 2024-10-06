//{"name":"B: Prime Subtractorization","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-1/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"2\n5\n8\n","output":"Case #1: 2\nCase #2: 3\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"prime_subtractorization_.*input[.]txt"},"output":{"type":"file","fileName":"prime_subtractorization_output.txt","pattern":null},"languages":{"java":{"taskClass":"BPrimeSubtractorization"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::consecutive_iter::ConsecutiveIter;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::misc::value_ref::ValueRef;
use algo_lib::numbers::primes::sieve::primes;
use algo_lib::value_ref;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    let primes = primes(10_000_000);
    let mut good = vec![2];
    for (&i, &j) in primes.consecutive_iter() {
        if i + 2 == j {
            good.push(i);
        }
    }
    value_ref!(Good GOOD: Vec<i32> = good);

    #[derive(Clone, Default)]
    struct Job {
        n: i32,
        ans: usize,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
        }

        fn solve(&mut self) {
            if self.n >= 5 {
                self.ans = Good::val().upper_bound(&(self.n - 2));
            }
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
