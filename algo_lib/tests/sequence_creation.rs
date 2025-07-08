//{"name":"Sequence Creation","group":"CodeChef - JAN21","url":"https://www.codechef.com/problems/ARCRT","interactive":false,"timeLimit":2500,"tests":[{"input":"2\n4\n2 2 3 2\n2 3 4 4\n5\n1 2 2 3 3\n2 1 2 3 1\n","output":"1 2 3 2\n2 1 1 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::euler_tour_tree::EulerTourForest;
use algo_lib::collections::id::Id;
use algo_lib::collections::payload::Payload;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::algebra::One;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);

    let mut id = Id::new();
    type Mod = ModInt7;
    let mut ans = Mod::one();
    let mut res = Vec::new();
    #[derive(Default)]
    struct Node(Option<(usize, usize)>);
    impl Payload for Node {
        const NEED_ACCUMULATE: bool = true;
        fn accumulate(&mut self, other: &Self) {
            self.0 = other.0;
        }
        fn reset_delta(&mut self) {}
    }
    let mut etf = EulerTourForest::new();

    let mut at = 0;
    for i in 0..n {
        let aa = id.get(a[i]);
        let bb = id.get(b[i]);
        if aa >= etf.node_count() {
            etf.add_node(Node::default());
        }
        if bb >= etf.node_count() {
            etf.add_node(Node::default());
        }

        loop {
            if etf.is_connected(aa, bb) {
                if etf.with_node(aa, |node| node.0.is_none()) {
                    break;
                }
            } else {
                if etf.with_node(aa, |node| node.0.is_none())
                    || etf.with_node(bb, |node| node.0.is_none())
                {
                    break;
                }
            }

            res.push(ans);

            let x = id.get(a[at]);
            let y = id.get(b[at]);
            at += 1;
            assert!(etf.is_connected(x, y));
            let marker = etf.with_node(x, |node| node.0);
            if marker == Some((x, y)) {
                etf.with_component_mut(x, |node| node.0 = None);
                if x != y {
                    ans /= Mod::new(2);
                }
                ans *= etf.component_size(x);
            } else if let Some((p, q)) = marker {
                etf.remove_edge(x, y);
                let xp = etf.is_connected(x, p);
                if xp == etf.is_connected(y, q) {
                    etf.add_edge(p, q, Node::default(), Node::default());
                    etf.with_component_mut(x, |node| node.0 = None);
                    ans /= Mod::new(2);
                    ans *= etf.component_size(x);
                } else if xp {
                    etf.with_component_mut(y, |node| node.0 = None);
                    ans *= etf.component_size(y);
                } else {
                    etf.with_component_mut(x, |node| node.0 = None);
                    ans *= etf.component_size(x);
                }
            } else {
                ans /= etf.component_size(x);
                etf.remove_edge(x, y);
                ans *= etf.component_size(x);
                ans *= etf.component_size(y);
            }
        }

        if etf.is_connected(aa, bb) {
            ans /= etf.component_size(aa);
            if aa != bb {
                ans *= Mod::new(2);
            }
            etf.with_component_mut(aa, |node| node.0 = Some((aa, bb)));
        } else {
            let m1 = etf.with_node(aa, |node| node.0);
            let m2 = etf.with_node(bb, |node| node.0);
            if m1.is_some() {
                if m1.unwrap().0 != m1.unwrap().1 {
                    ans /= Mod::new(2);
                }
            } else {
                ans /= etf.component_size(aa);
            }
            if m2.is_some() {
                if m2.unwrap().0 != m2.unwrap().1 {
                    ans /= Mod::new(2);
                }
            } else {
                ans /= etf.component_size(bb);
            }
            let marker = m1.or(m2);
            etf.add_edge(aa, bb, Node::default(), Node::default());
            etf.with_component_mut(aa, |node| node.0 = marker);
            if marker.is_some() {
                if marker.unwrap().0 != marker.unwrap().1 {
                    ans *= Mod::new(2);
                }
            } else {
                ans *= etf.component_size(aa);
            }
        }
    }

    while at < n {
        res.push(ans);

        let x = id.get(a[at]);
        let y = id.get(b[at]);
        at += 1;
        assert!(etf.is_connected(x, y));
        let marker = etf.with_node(x, |node| node.0);
        if marker == Some((x, y)) {
            etf.with_component_mut(x, |node| node.0 = None);
            if x != y {
                ans /= Mod::new(2);
            }
            ans *= etf.component_size(x);
        } else if let Some((p, q)) = marker {
            etf.remove_edge(x, y);
            let xp = etf.is_connected(x, p);
            if xp == etf.is_connected(y, q) {
                etf.add_edge(p, q, Node::default(), Node::default());
                etf.with_component_mut(x, |node| node.0 = None);
                ans /= Mod::new(2);
                ans *= etf.component_size(x);
            } else if xp {
                etf.with_component_mut(y, |node| node.0 = None);
                ans *= etf.component_size(y);
            } else {
                etf.with_component_mut(x, |node| node.0 = None);
                ans *= etf.component_size(x);
            }
        } else {
            ans /= etf.component_size(x);
            etf.remove_edge(x, y);
            ans *= etf.component_size(x);
            ans *= etf.component_size(y);
        }
    }
    out.print_line(res);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
    use algo_lib::misc::random::Random;
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
        let path = "./sequence_creation";
        let tl = 2500;
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
fn sequence_creation() {
    assert!(tester::run_tests());
}
