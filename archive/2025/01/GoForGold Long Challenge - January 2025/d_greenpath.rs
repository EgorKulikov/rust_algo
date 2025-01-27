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

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_traits::as_index::AsIndex;
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

    let inv = Mod::from_index(n).inv().unwrap();
    for _ in 0..q {
        let t = input.read_int();
        match t {
            1 => {
                let v = input.read_size() - 1;
                let d: Mod = input.read();
                st.update(
                    dfs_order.position[v]..=dfs_order.position[v],
                    &Node(d * Mod::from_index(n)),
                );
                let outer = d * Mod::from_index(dfs_order.len(v));
                st.update(..dfs_order.position[v], &Node(outer));
                st.update(dfs_order.end[v].., &Node(outer));
                total_add[v] += d;
                let id = hl.id[v];
                let pos = hl.pos[v];
                if hl.paths[id].len() > pos + 1 {
                    let child = hl.paths[id][pos + 1];
                    st.update(
                        dfs_order.subtree(child),
                        &Node(d * Mod::from_index(n - dfs_order.len(child))),
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
                        &Node(delta * Mod::from_index(n - dfs_order.len(vert))),
                    );
                }
                out.print_line(st.point_query(dfs_order.position[v]).0 * inv);
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

//START MAIN
#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
//END MAIN
