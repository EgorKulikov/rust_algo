//{"name":"Subtree swapping","group":"CodeChef - LTIME43","url":"https://www.codechef.com/problems/SBSWAP","interactive":false,"timeLimit":5000,"tests":[{"input":"10 5\n1 1 1 1 1 1 1 1 1 1\n1 2\n1 3\n1 8\n3 4\n8 9\n8 10\n4 5\n4 6\n4 7\n2 8 1\n1 3\n3 4 8\n1 3\n3 1 2\n","output":"5\n7\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::link_cut::LinkCutNode;
use algo_lib::collections::payload::Payload;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::dfs_order::DFSOrderTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::LegacyTestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let w = input.read_long_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    struct Node {
        val: i64,
        delta: i64,
        add_to_subtree_self: i64,
        add_to_subtree_all: i64,
        subtree_size: i64,
        subtree_size_delta: i64,
    }

    impl Payload for Node {
        const NEED_ACCUMULATE: bool = true;
        const NEED_UPDATE: bool = true;

        fn update(&mut self, left: Option<&Self>, right: Option<&Self>) {
            self.add_to_subtree_all = self.add_to_subtree_self
                + left.map_or(0, |node| node.add_to_subtree_all)
                + right.map_or(0, |node| node.add_to_subtree_all);
        }

        fn accumulate(&mut self, delta: &Self) {
            self.val += delta.delta;
            self.delta += delta.delta;
            self.subtree_size += delta.subtree_size_delta;
            self.subtree_size_delta += delta.subtree_size_delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
            self.subtree_size_delta = 0;
        }
    }

    let graph = Graph::with_biedges(n, &edges);
    let lca = graph.lca();
    let dfs_order = graph.dfs_order();
    let nodes = Vec::with_gen(n, |i| {
        LinkCutNode::new(Node {
            val: 0,
            delta: 0,
            add_to_subtree_self: 0,
            add_to_subtree_all: 0,
            subtree_size: (dfs_order.end[i] - dfs_order.position[i]) as i64,
            subtree_size_delta: 0,
        })
    });
    for i in 1..n {
        nodes[i].link(nodes[lca.parent(i).unwrap()]);
    }
    for i in 0..n {
        nodes[i].with_payload_mut(|node| {
            node.val += w[i];
            node.delta += w[i];
        });
    }

    for _ in 0..q {
        let t = input.read_int();

        match t {
            1 => {
                let u = input.read_size() - 1;
                out.print_line(nodes[u].with_payload(|p| {
                    p.val + (p.add_to_subtree_all - p.add_to_subtree_self) * p.subtree_size
                }));
            }
            2 => {
                let u = input.read_size() - 1;
                let x = input.read_long();
                nodes[u].with_payload_mut(|p| {
                    p.add_to_subtree_self += x;
                    p.val += p.subtree_size * x;
                    p.delta += p.subtree_size * x;
                });
            }
            3 => {
                let u = input.read_size() - 1;
                let v = input.read_size() - 1;
                let l = LinkCutNode::lca(nodes[u], nodes[v]).unwrap();
                if l == nodes[u] || l == nodes[v] {
                    out.print_line(-1);
                    continue;
                }
                let (u_val, u_size) = nodes[u].with_payload(|p| (p.val, p.subtree_size));
                let (v_val, v_size) = nodes[v].with_payload(|p| (p.val, p.subtree_size));
                let u_parent = nodes[u].parent().unwrap();
                u_parent.with_payload_mut(|p| {
                    p.val += v_val - u_val;
                    p.delta += v_val - u_val;
                    p.subtree_size += v_size - u_size;
                    p.subtree_size_delta += v_size - u_size;
                });
                let v_parent = nodes[v].parent().unwrap();
                v_parent.with_payload_mut(|p| {
                    p.val += u_val - v_val;
                    p.delta += u_val - v_val;
                    p.subtree_size += u_size - v_size;
                    p.subtree_size_delta += u_size - v_size;
                });
                nodes[u].cut();
                nodes[v].cut();
                nodes[u].link(v_parent);
                nodes[v].link(u_parent);
            }
            _ => unreachable!(),
        }
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
        let path = "./subtree_swapping";
        let tl = 5000;
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
fn subtree_swapping() {
    assert!(tester::run_tests());
}
