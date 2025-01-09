//{"name":"ucup_11_a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ucup_11_a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::{One, Zero};
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    type Mod = ModIntF;
    let mut matrix = Matrix::zero(49, 49);
    for i in 0..7 {
        for j in 0..7 {
            matrix[(i * 7 + j, i * 7 + j)] = Mod::new(50);
            if i < 6 {
                matrix[(i * 7 + j, (i + 1) * 7 + j)] = Mod::one();
            } else {
                matrix[(i * 7 + j, i * 7 + j)] += Mod::one();
            }
            if j < 6 {
                matrix[(i * 7 + j, i * 7 + j + 1)] = Mod::one();
            } else {
                matrix[(i * 7 + j, i * 7 + j)] += Mod::one();
            }
        }
    }
    let mut powers = Vec::with_capacity(30);
    let mut cur = matrix.clone();
    for _ in 0..30 {
        powers.push(cur.clone());
        cur = cur.mult(&cur);
    }

    let t = input.read_size();
    for _ in 0..t {
        let n = input.read_int();
        let mut res = Matrix::row(&[Mod::zero(); 49]);
        res[(0, 0)] = Mod::one();
        for i in 0..30 {
            if n.is_set(i) {
                res = res.mult(&powers[i]);
            }
        }
        out.print_line(res[(0, 48)]);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

mod tester {
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(dead_code)]

    use crate::{run, TASK_TYPE};
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::numbers::num_traits::bit_ops::BitOps;
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
            Box::new(1..=1)
        }

        fn input(&self, test: &Self::TestId, out: &mut Output) {
            out.print_line(5000);
            for _ in 0..5000 {
                out.print_line(usize::all_bits(30));
            }
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./ucup_11_a";
        let time_limit = 2000;
        let tester = match TASK_TYPE {
            crate::TaskType::Interactive => {
                Tester::new_interactive(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    std_interactor,
                )
                //Tester::new_interactive(time_limit, PRINT_LIMIT, path.to_string(), run, interact)
            }
            crate::TaskType::Classic => {
                Tester::new_classic(
                    time_limit,
                    PRINT_LIMIT,
                    path.to_string(),
                    run,
                    default_checker,
                )
                //Tester::new_classic(time_limit, PRINT_LIMIT, path.to_string(), run, check)
            }
        };
        let passed = tester.test_samples();
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn ucup_11_a() {
    assert!(tester::run_tests());
}
