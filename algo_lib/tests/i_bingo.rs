//{"name":"I. Bingo","group":"Universal Cup - The 3rd Universal Cup. Stage 16: Nanjing","url":"https://contest.ucup.ac/contest/1828/problem/9572","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2 2\n1 3 2 4\n3 1\n10 10 10\n1 3\n20 10 30\n3 4\n1 1 4 5 1 4 1 9 1 9 8 10\n","output":"56\n60\n60\n855346687\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IBingo"}}}

use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::mod_utils::Combinations;
use algo_lib::numbers::mod_int::prime_fft::PrimeFFT;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_unsigned_vec(n * m).sorted();

    type Mod = ModIntF;
    let c = Combinations::<Mod>::new(n * m + 1);
    let mut v = vec![Mod::zero(); n * m + 1];
    for i in 0..=n {
        for j in 0..=m {
            if i + j == 0 {
                continue;
            }
            v[i * m + j * n - i * j] += if (i + j) % 2 == 0 {
                Mod::zero() - Mod::one()
            } else {
                Mod::one()
            } * c.c(n, i)
                * c.c(m, j);
        }
    }
    let mut u = vec![Mod::zero(); n * m + 1];
    for i in 0..=n * m {
        v[i] *= c.fact(n * m - i);
        u[i] = c.inv_fact(i);
    }
    let res = PrimeFFT::new().multiply(&v, &u);
    let mut so_far = Mod::zero();
    let mut ans = Mod::zero();
    for i in 1..=n * m {
        if i != n * m && a[i] == a[i - 1] {
            continue;
        }
        let cur = res[i] * c.fact(i);
        ans += (cur - so_far) * Mod::new(a[i - 1]);
        so_far = cur;
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

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

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    struct MaxTest;

    impl GeneratedTestSet for MaxTest {
        type TestId = usize;

        fn tests(&self) -> Box<dyn Iterator<Item = Self::TestId>> {
            Box::new(1..=1)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {}

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./i_bingo";
        let tl = 1000;
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
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn i_bingo() {
    assert!(tester::run_tests());
}
