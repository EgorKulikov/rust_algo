//{"name":"Conga Line","group":"Kattis","url":"https://open.kattis.com/problems/congaline","interactive":false,"timeLimit":1000,"tests":[{"input":"3 6\namelia bubba\nkiryu coco\nollie udin\nPBBPFP\n","output":"bubba\ncoco\namelia\n\namelia\nbubba\nkiryu\ncoco\nollie\nudin\n"},{"input":"3 16\namelia bubba\nkiryu coco\nollie udin\nBRBPRFFPRBBBBBRP\n","output":"kiryu\nbubba\ncoco\n\nkiryu\nollie\nudin\nbubba\ncoco\namelia\n"},{"input":"3 22\namelia bubba\nkiryu coco\nollie udin\nBRBPRFFPRBBBBBRPCBBCFP\n","output":"kiryu\nbubba\ncoco\nollie\n\nollie\nudin\ncoco\nkiryu\namelia\nbubba\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CongaLine"}}}

use algo_lib::collections::treap::pure_payload::PurePayload;
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let _q = input.read_size();
    let names = input.read_str_vec(2 * n);
    let s = input.read_str();

    let mut treap = Tree::new();
    let mut ids = Vec::with_capacity(2 * n);
    for i in 0..2 * n {
        ids.push(treap.add_back(PurePayload(i)));
    }
    let mut mic = 0;
    for c in s {
        match c {
            b'F' => mic -= 1,
            b'B' => mic += 1,
            b'R' => {
                let cur = treap.range_index(mic..=mic).detach();
                treap.push_back(cur);
                if mic == 2 * n - 1 {
                    mic = 0;
                }
            }
            b'C' => {
                let cur = treap.range_index(mic..=mic);
                let id = cur.payload().as_ref().unwrap().0;
                let d = cur.detach();
                let left_size = treap.index_ref(&ids[id ^ 1]);
                let cur = treap.raise(&ids[id ^ 1]);
                if left_size < mic {
                    mic += 1;
                    if mic == 2 * n {
                        mic = 0;
                    }
                }
                cur.push_back(d);
            }
            b'P' => {
                out.print_line(
                    &names[treap.range_index(mic..=mic).payload().as_ref().unwrap().0 ^ 1],
                );
            }
            _ => unreachable!(),
        }
    }

    out.print_line(());
    for id in treap.iter() {
        out.print_line(&names[id.0]);
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

fn interact(mut sol_input: Input, mut sol_output: Output, mut input: Input) -> Result<(), String> {
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
    }

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

    fn input(&self, test: &Self::TestId, out: &mut Output) {
    }

    fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
        false
    }
}

pub(crate) fn run_tests() -> bool {
    let path = "./conga_line";
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
fn conga_line() {
    assert!(tester::run_tests());
}
