//{"name":"H. Guide Map","group":"Universal Cup - The 3rd Universal Cup. Stage 19: Shenyang","url":"https://contest.ucup.ac/contest/1865/problem/9805","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 4\n2 3\n","output":"6\n"},{"input":"2\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HGuideMap"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{Pushable, QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::dfs_order::{DFSOrder, DFSOrderTrait};
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 2).dec();

    let mut dsu = DSU::new(n);
    for (u, v) in edges.copy_iter() {
        dsu.join(u, v);
    }
    let mut v0 = Vec::new();
    let mut v1 = Vec::new();
    for i in 0..n {
        if dsu.get(i) == dsu.get(0) {
            v0.push(i);
        } else {
            v1.push(i);
        }
    }
    let mut g1 = Graph::new(v0.len());
    let mut g2 = Graph::new(v1.len());
    for (u, v) in edges.copy_iter() {
        if dsu.get(u) == dsu.get(0) {
            g1.add_edge(BiEdge::new(v0.lower_bound(&u), v0.lower_bound(&v)));
        } else {
            g2.add_edge(BiEdge::new(v1.lower_bound(&u), v1.lower_bound(&v)));
        }
    }
    type Mod = ModIntF;
    let mut c = vec![0; v0.len()];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, q: usize| {
        c[vert] = q;
        for e in &g1[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, q + v1.len() - v1.lower_bound(&v0[e.to()]));
        }
    });
    dfs.call(0, 0, 0);
    // let powers = powers(Mod::new(2), n + 1);
    let mut answers = Vec::new();
    for graph in [g1, g2] {
        #[derive(Default)]
        struct Node {
            children: Vec<usize>,
        }
        impl SegmentTreeNode for Node {
            fn new(_left: usize, _right: usize) -> Self {
                Self::default()
            }

            fn join(&mut self, _left_val: &Self, _right_val: &Self) {}

            fn accumulate(&mut self, _value: &Self) {}

            fn reset_delta(&mut self) {}
        }
        impl Pushable<&usize> for Node {
            fn push(&mut self, delta: &usize) {
                self.children.push(*delta);
            }
        }
        impl QueryResult<usize, usize> for Node {
            fn empty_result(_args: &usize) -> usize {
                0
            }

            fn result(&self, args: &usize) -> usize {
                self.children.len() - self.children.upper_bound(args)
            }

            fn join_results(
                left_res: usize,
                right_res: usize,
                _args: &usize,
                _left: usize,
                _mid: usize,
                _right: usize,
            ) -> usize {
                left_res + right_res
            }
        }
        let mut answer1 = vec![0; graph.vertex_count()];
        let mut answer3 = vec![0; graph.vertex_count()];
        let DFSOrder { position, end } = graph.dfs_order();
        let mut st = SegmentTree::<Node>::new(graph.vertex_count());
        for i in 0..graph.vertex_count() {
            st.point_through_update(position[i], &i);
        }
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> usize {
            let mut cur = 0;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                cur += f.call(e.to(), vert);
            }
            answer1[vert] = cur;
            cur += st.query_with_args(position[vert]..end[vert], &vert);
            answer3[vert] = cur;
            cur
        });
        dfs.call(0, 0);
        let mut answer2 = vec![0; graph.vertex_count()];
        let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, additional: usize| {
            answer2[vert] = answer1[vert] + additional;
            for e in &graph[vert] {
                if e.to() == prev {
                    continue;
                }
                let call_add = additional + answer1[vert] - answer3[e.to()]
                    + st.query_with_args(..position[e.to()], &vert)
                    + st.query_with_args(end[e.to()].., &vert);
                f.call(e.to(), vert, call_add);
            }
        });
        dfs.call(0, 0, 0);
        answers.push(answer2);
    }
    let mut ans = Mod::zero();
    if v1.len() == 1 {
        ans += Mod::new(2).power(answers[0][0]);
    }
    let mut by = Mod::zero();
    for i in v1.indices() {
        by += Mod::new(2).power(answers[0][0] + answers[1][i] + v1.len() - i - 1);
    }
    for i in v0.indices() {
        ans += by * Mod::new(2).power(c[i]);
    }
    out.print_line(ans);
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
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
