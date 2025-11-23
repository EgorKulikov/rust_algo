//{"name":"P5 - Coffee Jelly","group":"DMOJ - OTHS Coding Competition 2","url":"https://dmoj.ca/problem/othscc2p5","interactive":false,"timeLimit":1000,"tests":[{"input":"6 10\nXXXXXXXXXX\nX.*.X..X.X\nXX.XXXXXXX\nXXXX..XXXX\n*.......XX\nXXXXXXXXXX\n","output":"2\n"},{"input":"7 6\nXXX..X\n..XXX*\nXXXXXX\n..*...\nXXXXXX\nXXXXX.\n......\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P5CoffeeJelly"}}}

use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::{TaskType, LegacyTestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let map = input.read_char_table(n, m);

    let mut visited = Arr2d::new(n, m, false);
    let mut queue = Vec::new();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if map[(i, j)] != b'X' && !visited[(i, j)] {
                queue.push((i, j));
                visited[(i, j)] = true;
                let mut good = true;
                while let Some((r, c)) = queue.pop() {
                    if map[(r, c)] == b'*' {
                        good = false;
                    }
                    for (nr, nc) in D4::iter(r, c, n, m) {
                        if !visited[(nr, nc)] && map[(nr, nc)] != b'X' {
                            visited[(nr, nc)] = true;
                            queue.push((nr, nc));
                        }
                    }
                }
                if good {
                    ans += 1;
                }
            }
        }
    }
    out.print_line(ans);
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
        let path = "./p5_coffee_jelly";
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
        // tester.test_generated("Stress test", false, StressTest);
        passed
    }
}
#[test]
fn p5_coffee_jelly() {
    assert!(tester::run_tests());
}
