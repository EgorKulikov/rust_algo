//{"name":"D: Tower Rush","group":"Meta Coding Competitions - Meta Hacker Cup 2023 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-2/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"2\n5 3 3\n2 4 5 7 8\n3 2 4\n3 6 9\n","output":"Case #1: 54\nCase #2: 0\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"tower_rush_.*input[.]txt"},"output":{"type":"file","fileName":"tower_rush_output.txt","pattern":null},"languages":{"java":{"taskClass":"DTowerRush"}}}

use algo_lib::collections::default_map::default_hash_map::DefaultHashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};
use algo_lib::numbers::gcd::gcd;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::zero_one::ZeroOne;
use algo_lib::numbers::num_utils::factorials;
use algo_lib::numbers::primes::sieve::divisor_table;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _data: &PreCalc) {
    type Mod = ModInt7;

    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        k: usize,
        d: usize,
        h: Vec<usize>,
        ans: Mod,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read_size();
            self.k = input.read_size();
            self.d = input.read_size();
            self.h = input.read_vec(self.n);
        }

        fn solve(&mut self) {
            let div = divisor_table(1000001);
            let primes = |mut n: usize| -> Vec<usize> {
                let mut res = Vec::new();
                while n > 1 {
                    let p = div[n];
                    res.push(p);
                    while n % p == 0 {
                        n /= p;
                    }
                }
                res
            };
            let q_primes = |mut n: usize| -> usize {
                let mut res = 0;
                while n > 1 {
                    let p = div[n];
                    res += 1;
                    while n % p == 0 {
                        n /= p;
                    }
                }
                res
            };
            let mut qty = DefaultHashMap::<_, usize>::new();
            for &i in &self.h {
                let n = i / gcd(i, self.d);
                let p = primes(n);
                for j in 1..(1 << p.len()) {
                    let mut prod = 1;
                    for k in 0..p.len() {
                        if j & (1 << k) != 0 {
                            prod *= p[k];
                        }
                    }
                    qty[prod] += 1;
                }
            }

            let fact = factorials::<Mod>(self.n + 1);
            self.ans = fact[self.n] / fact[self.n - self.k];
            for (p, q) in qty {
                if q < self.k {
                    continue;
                }
                let sign = if q_primes(p) % 2 == 0 {
                    Mod::one()
                } else {
                    -Mod::one()
                };
                self.ans += sign * fact[q] / fact[q - self.k];
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
