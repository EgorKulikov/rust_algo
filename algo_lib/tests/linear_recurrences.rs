//{"name":"Linear Recurrences","group":"Kattis","url":"https://open.kattis.com/problems/linearrecurrence","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n0 1 1\n0 1\n6\n1 100000\n2 100000\n3 100000\n4 100000\n5 100000\n6 100000\n","output":"1\n1\n2\n3\n5\n8\n"},{"input":"2\n5 7 9\n36713 5637282\n4\n1 10000\n1375 1\n3781 23\n34683447233 1571385\n","output":"7282\n0\n16\n299255\n"},{"input":"3\n1 2 3 4\n0 0 0\n1\n42424242424242 1000000\n","output":"552200\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LinearRecurrences"}}}

use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::dynamic_value;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::LegacyTestType;
use algo_lib::misc::value::DynamicValue;
use algo_lib::numbers::matrix::Matrix;
use algo_lib::numbers::mod_int::ModInt;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_unsigned_vec(n + 1);
    let x = input.read_unsigned_vec(n);

    dynamic_value!(Modulo: u32);
    let q = input.read_size();
    for _ in 0..q {
        let t = input.read_size();
        let m = input.read_unsigned();
        Modulo::set(m);
        type Mod = ModInt<Modulo>;
        let mut matrix = Matrix::zero(n + 1, n + 1);
        for i in 0..n {
            matrix[(0, i)] = Mod::new(a[i + 1]);
        }
        matrix[(0, n)] = Mod::new(a[0]);
        for i in 1..n {
            matrix[(i, i - 1)] = Mod::new(1);
        }
        matrix[(n, n)] = Mod::new(1);
        let res = matrix.power(t);
        let mut vec = Matrix::zero(n + 1, 1);
        for i in 0..n {
            vec[(i, 0)] = Mod::new(x[Back(i)]);
        }
        vec[(n, 0)] = Mod::new(1);
        let ans = Matrix::mult(&res, &vec)[(n - 1, 0)];
        out.print_line(ans);
    }
}

pub static TEST_TYPE: LegacyTestType = LegacyTestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        LegacyTestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        LegacyTestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        LegacyTestType::MultiEof => {
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
        let path = "./linear_recurrences";
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
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn linear_recurrences() {
    assert!(tester::run_tests());
}
