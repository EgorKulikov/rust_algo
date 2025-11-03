//{"name":"10 Kinds of People","group":"Kattis","url":"https://open.kattis.com/problems/10kindsofpeople","interactive":false,"timeLimit":1000,"tests":[{"input":"1 4\n1100\n2\n1 1 1 4\n1 1 1 1\n","output":"neither\ndecimal\n"},{"input":"10 20\n11111111111111111111\n11000000000000000101\n11111111111111110000\n11111111111111110000\n11000000000000000111\n00011111111111111111\n00111111111111111111\n10000000000000001111\n11111111111111111111\n11111111111111111111\n3\n2 3 8 16\n8 1 7 3\n1 1 10 20\n","output":"binary\ndecimal\nneither\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KindsOfPeople"}}}

use algo_lib::collections::dsu2d::DSU2d;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::LegacyTaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let r = input.read_size();
    let c = input.read_size();
    let map = input.read_char_table(r, c);

    let mut dsu = DSU2d::new(r, c);
    for i in 0..r {
        for j in 0..c {
            for (x, y) in D4::iter(i, j, r, c) {
                if map[(i, j)] == map[(x, y)] {
                    dsu.union(i, j, x, y);
                }
            }
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let r1 = input.read_size() - 1;
        let c1 = input.read_size() - 1;
        let r2 = input.read_size() - 1;
        let c2 = input.read_size() - 1;
        out.print_line(
            match (dsu.find(r1, c1) == dsu.find(r2, c2), map[(r1, c1)]) {
                (true, b'0') => "binary",
                (true, b'1') => "decimal",
                _ => "neither",
            },
        );
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
        let path = "./kinds_of_people";
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
fn kinds_of_people() {
    assert!(tester::run_tests());
}
