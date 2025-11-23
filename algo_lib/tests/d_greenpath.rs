//{"name":"D. Greenpath","group":"Codeforces - GoForGold Long Challenge - January 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/579716/problem/D","interactive":false,"timeLimit":5000,"tests":[{"input":"5 12\n1 2\n1 3\n2 4\n2 5\n1 1 1\n2 1\n2 2\n2 3\n2 4\n2 5\n1 2 2\n2 1\n2 2\n2 3\n2 4\n2 5\n","output":"1\n199648871\n399297742\n199648871\n199648871\n598946614\n199648873\n2\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::DefaultHashMap;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::dfs_order::DFSOrderTrait;
use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::LegacyTestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::invertible::Invertible;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    type Mod = ModIntF;
    let graph = Graph::with_biedges(n, &edges);
    let dfs_order = graph.dfs_order();
    let hl = graph.hl_decomposition();
    #[derive(Default, Clone)]
    struct Node(Mod);
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            self.0 += value.0;
        }
        fn reset_delta(&mut self) {
            self.0 = Mod::zero();
        }
    }
    let mut st = SegmentTree::<Node>::new(n);
    let mut total_add = vec![Mod::zero(); n];
    let mut pushed = vec![DefaultHashMap::new(Mod::zero()); n];

    let inv = Mod::from(n).inv().unwrap();
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let v = input.read_size() - 1;
                let d: Mod = input.read();
                st.update(dfs_order.position[v]..=dfs_order.position[v], &Node(d * n));
                let outer = d * dfs_order.len(v);
                st.update(..dfs_order.position[v], &Node(outer));
                st.update(dfs_order.end[v].., &Node(outer));
                total_add[v] += d;
                let id = hl.id[v];
                let pos = hl.pos[v];
                if hl.paths[id].len() > pos + 1 {
                    let child = hl.paths[id][pos + 1];
                    st.update(
                        dfs_order.subtree(child),
                        &Node(d * (n - dfs_order.len(child))),
                    );
                }
            }
            2 => {
                let v = input.read_size() - 1;
                let segs = hl.iter(v..=0).collect::<Vec<_>>().do_with(|s| s.reverse());
                for seg in segs.iter_skip(1) {
                    let id = seg.id;
                    let vert = hl.paths[id][0];
                    let parent = hl.parent[vert];
                    let delta = total_add[parent] - pushed[parent][vert];
                    pushed[parent][vert] += delta;
                    st.update(
                        dfs_order.subtree(vert),
                        &Node(delta * (n - dfs_order.len(vert))),
                    );
                }
                out.print_line(st.point_query(dfs_order.position[v]).0 * inv);
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
        let path = "./d_greenpath";
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
fn d_greenpath() {
    assert!(tester::run_tests());
}
