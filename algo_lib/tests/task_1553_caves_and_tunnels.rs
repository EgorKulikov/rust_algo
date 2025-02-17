//{"name":"1553. Caves and Tunnels","group":"Timus Online Judge - Novosibirsk SU Contest. Petrozavodsk training camp, September 2007","url":"https://acm.timus.ru/problem.aspx?space=1&num=1553","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n1 2\n2 3\n2 4\n6\nI 1 1\nG 1 1\nG 3 4\nI 2 3\nG 1 1\nG 3 4\n","output":"1\n0\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::link_cut::LinkCutNode;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::Payload;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let lca = graph.lca();
    struct Node {
        level: i64,
        max_level: i64,
    }
    impl Payload for Node {
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.max_level = self.level;
            if let Some(left) = left {
                self.max_level.maxim(left.max_level);
            }
            if let Some(right) = right {
                self.max_level.maxim(right.max_level);
            }
        }
    }
    let nodes = Vec::with_gen(n, |_| {
        LinkCutNode::new(Node {
            level: 0,
            max_level: 0,
        })
    });
    for i in 1..n {
        nodes[i].link(nodes[lca.parent(i).unwrap()]);
    }

    let q = input.read_size();
    for _ in 0..q {
        let c = input.read_char();
        match c {
            b'I' => {
                let u = input.read_size() - 1;
                let v = input.read_long();
                nodes[u].with_payload_mut(|node| {
                    node.level += v;
                });
            }
            b'G' => {
                let u = input.read_size() - 1;
                let v = input.read_size() - 1;
                let l = lca.lca(u, v);
                nodes[l].cut();
                let ans = nodes[u]
                    .with_payload(|p| p.max_level)
                    .max(nodes[v].with_payload(|p| p.max_level));
                if l != 0 {
                    nodes[l].link(nodes[lca.parent(l).unwrap()]);
                }
                out.print_line(ans);
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
        let path = "./task_1553_caves_and_tunnels";
        let tl = 3000;
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
fn task_1553_caves_and_tunnels() {
    assert!(tester::run_tests());
}
