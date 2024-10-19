//{"name":"A. Next","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n+ 1\n+ 3\n+ 3\n? 2\n+ 1\n? 4\n","output":"3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANext"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();

    let mut set = TreapSet::new();

    let mut delta = None;
    for _ in 0..n {
        match input.read_char() {
            b'+' => {
                let i = input.read_long();
                if let Some(delta) = delta {
                    set.insert((i + delta + 1000000000) % 1000000000);
                } else {
                    set.insert(i);
                }
                delta = None;
            }
            b'?' => {
                let i = input.read_long();
                let res = set.ceil(&i);
                if let Some(&i) = res {
                    delta = Some(i);
                } else {
                    delta = Some(-1);
                }
                out.print_line(res);
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
    use algo_lib::collections::slice_ext::bounds::Bounds;
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
            let n = 10;
            out.print_line(n);
            for _ in 0..n {
                if random().next(2) == 0 {
                    out.print_line(('+', random().next(1000000001)));
                } else {
                    out.print_line(('?', random().next(1000000001)));
                }
            }
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            let n = input.read_size();

            let mut set = Vec::new();
            let mut delta = None;
            for _ in 0..n {
                match input.read_char() {
                    b'+' => {
                        let i = input.read_long();
                        if let Some(delta) = delta {
                            set.push((i + delta + 1000000000) % 1000000000);
                        } else {
                            set.push(i);
                        }
                        delta = None;
                    }
                    b'?' => {
                        let i = input.read_long();
                        set.sort();
                        set.dedup();
                        let res = set.get(set.lower_bound(&i));
                        if let Some(&i) = res {
                            delta = Some(i);
                        } else {
                            delta = Some(-1);
                        }
                        out.print_line(res);
                    }
                    _ => unreachable!(),
                }
            }
            true
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./a_next";
        let time_limit = 1000;
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
        tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn a_next() {
    assert!(tester::run_tests());
}
