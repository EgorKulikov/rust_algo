//{"name":"2048","group":"Kattis","url":"https://open.kattis.com/problems/2048","interactive":false,"timeLimit":1000,"tests":[{"input":"2 0 0 2\n4 16 8 2\n2 64 32 4\n1024 1024 64 0\n0\n","output":"4 0 0 0\n4 16 8 2\n2 64 32 4\n2048 64 0 0\n"},{"input":"2 0 0 2\n4 16 8 2\n2 64 32 4\n1024 1024 64 0\n1\n","output":"2 16 8 4\n4 64 32 4\n2 1024 64 0\n1024 0 0 0\n"},{"input":"2 0 0 2\n4 16 8 2\n2 64 32 4\n1024 1024 64 0\n2\n","output":"0 0 0 4\n4 16 8 2\n2 64 32 4\n0 0 2048 64\n"},{"input":"2 0 0 2\n4 16 8 2\n2 64 32 4\n1024 1024 64 0\n3\n","output":"2 0 0 0\n4 16 8 0\n2 64 32 4\n1024 1024 64 4\n"},{"input":"2 2 4 8\n4 0 4 4\n16 16 16 16\n32 16 16 32\n0\n","output":"4 4 8 0\n8 4 0 0\n32 32 0 0\n32 32 32 0\n"},{"input":"2 2 4 8\n4 0 4 4\n16 16 16 16\n32 16 16 32\n2\n","output":"0 4 4 8\n0 0 4 8\n0 0 32 32\n0 32 32 32\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut grid = input.read_int_table(4, 4);
    let dir = input.read_size();

    for _ in 0..dir {
        grid = grid.rotate_counterclockwise();
    }
    for i in 0..4 {
        let elements = grid
            .row(i)
            .filter(|&&x| x != 0)
            .copied()
            .collect::<Vec<_>>();
        let mut j = 0;
        for k in 0..4 {
            if j == elements.len() {
                grid[(i, k)] = 0;
            } else if j + 1 == elements.len() || elements[j] != elements[j + 1] {
                grid[(i, k)] = elements[j];
                j += 1;
            } else {
                grid[(i, k)] = elements[j] * 2;
                j += 2;
            }
        }
    }
    for _ in 0..(4 - dir) % 4 {
        grid = grid.rotate_counterclockwise();
    }
    out.print_line(grid);
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
        let path = "./task";
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
fn task() {
    assert!(tester::run_tests());
}
