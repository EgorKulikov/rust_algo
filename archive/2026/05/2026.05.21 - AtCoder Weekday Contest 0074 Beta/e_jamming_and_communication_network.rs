use algo_lib::collections::dsu::DSU;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::hl_decomposition::HLDecompositionTrait;
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input
        .read_vec::<(usize, usize, i64)>(m)
        .dec()
        .sorted_by_key(|x| x.2);

    let mut graph = Graph::new(n, n - 1);
    let mut dsu = DSU::new(n);
    let mut add = Vec::with_capacity(m - n + 1);
    let mut base = 0;
    for (u, v, w) in edges {
        if dsu.union(u, v) {
            graph.add_edge(BiWeightedEdge::new(u, v, w));
            base += w;
        } else {
            add.push((u, v, w));
        }
    }
    let hl = graph.hl_decomposition();
    #[derive(Clone)]
    struct Node {
        val: i64,
        removed: bool,
    }
    impl Default for Node {
        fn default() -> Self {
            Self {
                val: i64::MAX,
                removed: false,
            }
        }
    }
    impl Node {
        fn get_val(&self) -> i64 {
            if self.removed {
                i64::MAX
            } else {
                self.val
            }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.get_val().min(right_val.get_val());
        }
        fn accumulate(&mut self, value: &Self) {
            if value.removed {
                self.removed = true;
            }
        }
        fn reset_delta(&mut self) {}
    }
    let mut st = Vec::with_gen(hl.paths.len(), |i| {
        SegmentTree::with_gen(hl.paths[i].len() - 1, |j| {
            let vertex = hl.paths[i][j + 1];
            let parent = hl.paths[i][j];
            for edge in graph.adj(vertex) {
                if edge.to() == parent {
                    return Node {
                        val: edge.weight(),
                        removed: false,
                    };
                }
            }
            unreachable!();
        })
    });
    let mut up = Vec::with_gen(hl.paths.len(), |i| {
        let vert = hl.paths[i][0];
        let parent = hl.parent[vert];
        for edge in graph.adj(vert) {
            if edge.to() == parent {
                return edge.weight();
            }
        }
        i64::MAX
    });
    let lca = graph.lca();
    let mut ans = base;
    for (u, v, w) in add {
        let l = lca.lca(u, v);
        let mut min = i64::MAX;
        for v in [u, v] {
            for path in hl.iter(v..=l) {
                let cur = st[path.id].query(path.pos_from..path.pos_to).get_val();
                st[path.id].update(
                    path.pos_from..path.pos_to,
                    &Node {
                        removed: true,
                        ..Default::default()
                    },
                );
                min.minim(cur);
                if hl.paths[path.id][path.pos_from] != l {
                    min.minim(up[path.id]);
                    up[path.id] = i64::MAX;
                }
            }
        }
        if min != i64::MAX {
            ans.maxim(base + w - min);
        }
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
}

#[cfg(feature = "local")]
mod tester;

#[cfg(feature = "local")]
fn main() {
    tester::run_tests();
}

#[cfg(not(feature = "local"))]
fn main() {
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
