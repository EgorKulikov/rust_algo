//{"name":"Dynamic Tree Subtree Add Subtree Sum","group":"Library Checker","url":"https://judge.yosupo.jp/problem/dynamic_tree_subtree_add_subtree_sum","interactive":false,"timeLimit":5000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::euler_tour_tree::EulerTourForest;
use algo_lib::collections::payload::Payload;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::LegacyTaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1);

    #[derive(Default)]
    struct Node {
        val: i64,
        sum: i64,
        delta: i64,
        size: i64,
        self_size: i64,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;
        const NEED_ACCUMULATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            assert!(self.delta == 0);
            self.size = self.self_size + left.map_or(0, |l| l.size) + right.map_or(0, |r| r.size);
            self.sum = self.val + left.map_or(0, |l| l.sum) + right.map_or(0, |r| r.sum);
        }

        fn accumulate(&mut self, delta: &Self) {
            self.val += delta.delta * self.self_size;
            self.sum += delta.delta * self.size;
            self.delta += delta.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut etf = EulerTourForest::new();
    for i in 0..n {
        etf.add_node(Node {
            val: a[i],
            sum: a[i],
            size: 1,
            self_size: 1,
            ..Node::default()
        });
    }
    for (u, v) in edges {
        etf.add_edge(u, v, Node::default(), Node::default());
    }

    for _ in 0..q {
        let t = input.read_int();
        match t {
            0 => {
                let u = input.read_size();
                let v = input.read_size();
                let w = input.read_size();
                let x = input.read_size();
                etf.remove_edge(u, v);
                etf.add_edge(w, x, Node::default(), Node::default());
            }
            1 => {
                let v = input.read_size();
                let p = input.read_size();
                let x = input.read_long();
                etf.with_subtree_mut(v, p, |node| {
                    node.accumulate(&Node {
                        delta: x,
                        ..Node::default()
                    });
                });
            }
            2 => {
                let v = input.read_size();
                let p = input.read_size();
                out.print_line(etf.with_subtree(v, p, |node| node.sum));
            }
            _ => unreachable!(),
        }
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
    #![allow(unused_imports)]

    use crate::{run, TASK_TYPE};
    use algo_lib::io::input::Input;
    use algo_lib::io::output::Output;
    use algo_lib::misc::random::Random;
    use tester::classic::default_checker;
    use tester::classic::EPS;
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
            let mut r = Random::new();
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
            let mut r = Random::new();
        }

        fn output(&self, test: &Self::TestId, input: &mut Input, out: &mut Output) -> bool {
            false
        }
    }

    pub(crate) fn run_tests() -> bool {
        let path = "./dynamic_tree_subtree_add_subtree_sum";
        let tl = 5000;
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
fn dynamic_tree_subtree_add_subtree_sum() {
    assert!(tester::run_tests());
}
