//{"name":"A+B Problem","group":"Kattis","url":"https://open.kattis.com/problems/aplusb","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n1 2 3 4\n","output":"4\n"},{"input":"6\n1 1 3 3 4 6\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ABProblem"}}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::convolution::convolution;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    const HALF: usize = 50_000;
    let mut b = vec![0; HALF * 2 + 1];
    for x in a.copy_iter() {
        b[(x + HALF as i32) as usize] += 1;
    }
    let mut c = convolution(&b, &b);
    for x in a.copy_iter() {
        c[(2 * x + 2 * HALF as i32) as usize] -= 1;
    }
    let mut ans = 0;
    for x in a.copy_iter() {
        ans += c[(x + 2 * HALF as i32) as usize];
    }
    let zeroes = a.copy_filter(|&x| x == 0).count();
    ans -= zeroes as i128 * (n - 1) as i128 * 2;
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

#[cfg(test)]
mod test {
    use algo_lib::numbers::primes::prime::is_prime;

    #[test]
    fn test() {
        let mut p = 1_000_000_000_000_000_000i64;
        loop {
            let rem = p % (1 << 20);
            p += (1 << 20) - rem + 1;
            if is_prime(p) {
                println!("{} is prime", p);
                break;
            }
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]
    #![allow(unused_imports)]

    use crate::{run, TASK_TYPE};
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::random::random;
    use tester::classic::default_checker;
    use tester::interactive::std_interactor;
    use tester::test_set::GeneratedTestSet;
    use tester::Tester;

    const PRINT_LIMIT: usize = 1000;

    fn interact(
        mut sol_input: Input,
        mut sol_output: Output,
        mut input: Input,
    ) -> Result<(), String> {
        Ok(())
    }

    fn check(mut input: Input, expected: Option<Input>, mut output: Input) -> Result<(), String> {
        Ok(())
    }

    struct StressTest;

    impl GeneratedTestSet for StressTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            let n = random().gen_range(1..=5);
            let mut a = Vec::with_capacity(n);
            for _ in 0..n {
                a.push(random().gen_range(-5..=5));
            }
            out.print_line(n);
            out.print_line(a);
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            let n = input.read();
            let a = input.read_int_vec(n);
            let mut ans = 0;
            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    for k in 0..n {
                        if i == k || j == k {
                            continue;
                        }
                        if a[i] + a[j] == a[k] {
                            ans += 1;
                        }
                    }
                }
            }
            out.print_line(ans);
            true
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> impl Iterator<Item = Self::TestId> {
            1..=1
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./a_bproblem";
        let tl = 4000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
                // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn a_bproblem() {
    assert!(tester::run_tests());
}
