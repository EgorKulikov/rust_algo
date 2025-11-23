//{"name":"K. Река","group":"Codeforces - Treaps","url":"https://codeforces.com/gym/539514/problem/K","interactive":false,"timeLimit":3000,"tests":[{"input":"4 0\n3 5 5 4\n5\n1 1\n2 1\n1 3\n2 2\n1 3\n","output":"75\n105\n73\n101\n83\n113\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KReka"}}}

use algo_lib::collections::payload::Payload;
use algo_lib::collections::treap::Tree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, LegacyTestType};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    struct Node {
        len: usize,
        sum_sq: usize,
    }

    impl Node {
        fn new(len: usize) -> Self {
            Self {
                len,
                sum_sq: len * len,
            }
        }
    }

    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.sum_sq =
                self.len * self.len + left.map_or(0, |l| l.sum_sq) + right.map_or(0, |r| r.sum_sq);
        }
    }

    fn push(node: &mut Node, delta: usize) {
        node.sum_sq -= node.len * node.len;
        node.len += delta;
        node.sum_sq += node.len * node.len;
    }

    let n = input.read_size();
    input.read_size();
    let a = input.read_size_vec(n);

    let mut treap = Tree::new();
    for a in a {
        treap.add_back(Node::new(a));
    }

    out.print_line(treap.payload().map(|p| p.sum_sq));

    let k = input.read_size();
    for _ in 0..k {
        let e = input.read_int();
        let id = input.read_size() - 1;

        match e {
            1 => {
                let size = treap.size();
                let view = treap.range_index(id..=id);
                let len = view.payload().unwrap().len;
                view.detach();
                if id == 0 {
                    treap.range_index(..1).payload_mut().map(|p| push(p, len));
                } else if id == size - 1 {
                    treap
                        .range_index(size - 2..)
                        .payload_mut()
                        .map(|p| push(p, len));
                } else {
                    treap
                        .range_index(id - 1..id)
                        .payload_mut()
                        .map(|p| push(p, len / 2));
                    treap
                        .range_index(id..id + 1)
                        .payload_mut()
                        .map(|p| push(p, len - len / 2));
                }
            }
            2 => {
                let view = treap.range_index(id..=id);
                let len = view.payload().unwrap().len;
                view.detach();
                view.add_back(Node::new(len / 2));
                view.add_back(Node::new(len - len / 2));
            }
            _ => unreachable!(),
        }
        out.print_line(treap.payload().map(|p| p.sum_sq));
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
        let path = "./k_reka";
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
fn k_reka() {
    assert!(tester::run_tests());
}
