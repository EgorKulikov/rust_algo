//{"name":"A. Two out of Three","group":"Yandex - Yandex Cup 2024 — Algorithm — Qualification","url":"https://contest.yandex.com/contest/69390/problems/","interactive":false,"timeLimit":1000,"tests":[{"input":"2 3 5\n10\n","output":"42\n"},{"input":"5 6 7\n1000000\n","output":"13999986\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATwoOutOfThree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::LegacyTaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::gcd::lcm;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let mut n = input.read_long() - 1;

    let ab = lcm(a, b);
    let bc = lcm(b, c);
    let ac = lcm(a, c);
    let period = lcm(ab, c);
    let per_period = period / ab - 1 + period / bc - 1 + period / ac - 1;
    if per_period == 0 {
        out.print_line(-1);
        return;
    }
    let full_periods = n / per_period;
    let (x, y) = full_periods.overflowing_mul(period);
    if y || x > 1000_000_000_000_000_000 {
        out.print_line(-1);
        return;
    }
    n -= full_periods * per_period;
    let mut n1 = ab;
    let mut n2 = bc;
    let mut n3 = ac;
    loop {
        if n == 0 {
            let ans = n1.min(n2).min(n3) + x;
            if ans <= 1000_000_000_000_000_000 {
                out.print_line(ans);
            } else {
                out.print_line(-1);
            }
            return;
        }
        n -= 1;
        if n1 < n2 && n1 < n3 {
            n1 += ab;
        } else if n2 < n1 && n2 < n3 {
            n2 += bc;
        } else {
            n3 += ac;
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: LegacyTaskType = LegacyTaskType::Classic;

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
        LegacyTaskType::Classic => input.is_empty(),
        LegacyTaskType::Interactive => true,
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

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
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
        let path = "./a_two_out_of_three";
        let tl = 1000;
        let tester = match TASK_TYPE {
            crate::LegacyTaskType::Interactive => {
                Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, std_interactor)
                // Tester::new_interactive(tl, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::LegacyTaskType::Classic => {
                Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, default_checker)
                // Tester::new_classic(tl, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Max test", true, MaxTest);
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn a_two_out_of_three() {
    assert!(tester::run_tests());
}
