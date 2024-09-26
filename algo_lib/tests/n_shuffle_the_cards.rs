//{"name":"N. Shuffle the Cards","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/N","interactive":false,"timeLimit":1500,"tests":[{"input":"10 2\n6 2 2\n5 3 6\n","output":"1 2 8 7 3 9 6 5 4 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"NShuffleTheCards"}}}

use algo_lib::collections::treap::{PurePayload, Treap};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut treap = Treap::new();
    for i in 1..=n {
        treap.add(PurePayload(i));
    }

    for _ in 0..m {
        let a = input.read_size();
        let b = input.read_size();
        let c = input.read_size();

        let (a_cards, deck) = treap.split_at(a);
        let (mut b_cards, deck) = deck.split_at(b);
        let deck = Treap::merge(a_cards, deck);
        let (c_cards, deck) = deck.split_at(c);
        b_cards.reverse();
        let deck = Treap::merge(b_cards, deck);
        treap = Treap::merge(c_cards, deck);
    }
    out.print_line_iter(treap.iter().map(|payload| payload.0));
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

pub(crate) fn run_tests() -> bool {
    let path = "./n_shuffle_the_cards";
    let time_limit = 1500;
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
fn n_shuffle_the_cards() {
    assert!(tester::run_tests());
}
