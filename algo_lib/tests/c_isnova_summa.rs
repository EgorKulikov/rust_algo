//{"name":"C. И снова сумма...","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n+ 1\n+ 3\n+ 3\n? 2 4\n+ 1\n? 2 4\n","output":"3\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CISnovaSumma"}}}

use algo_lib::collections::treap::payload::{OrdPayload, Payload};
use algo_lib::collections::treap::Treap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    struct Node {
        value: i64,
        sum: i64,
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum = self.value + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
        }
    }

    impl OrdPayload for Node {
        type Key = i64;
        fn key(&self) -> &Self::Key {
            &self.value
        }
    }

    let mut treap = Treap::new();
    let n = input.read_size();
    let mut delta = None;

    for _ in 0..n {
        match input.read_char() {
            b'+' => {
                let mut value = input.read();
                if let Some(delta) = delta {
                    value += delta;
                    value %= 1000000000;
                }
                delta = None;
                treap.insert(Node { value, sum: value });
            }
            b'?' => {
                let l = input.read_long();
                let r = input.read_long();
                let sum = treap.range(&l..=&r).payload().map(|p| p.sum).unwrap_or(0);
                delta = Some(sum);
                out.print_line(sum);
            }
            _ => unreachable!(),
        }
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

    pub(crate) fn run_tests() -> bool {
        let path = "./c_isnova_summa";
        let time_limit = 3000;
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
fn c_isnova_summa() {
    assert!(tester::run_tests());
}
