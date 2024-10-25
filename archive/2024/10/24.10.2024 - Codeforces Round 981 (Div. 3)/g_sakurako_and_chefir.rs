//{"name":"G. Sakurako and Chefir","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n5\n1 2\n2 3\n3 4\n3 5\n3\n5 1\n3 1\n2 0\n9\n8 1\n1 7\n1 4\n7 3\n4 9\n3 2\n1 5\n3 6\n7\n6 0\n2 3\n6 2\n8 2\n2 4\n9 2\n6 3\n6\n2 1\n2 5\n2 4\n5 6\n4 3\n3\n3 1\n1 3\n6 5\n","output":"2 1 2\n0 5 2 4 5 5 5\n1 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSakurakoAndChefir"}}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{
    Callable2, Callable3, RecursiveFunction2, RecursiveFunction3,
};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();
    let q = input.read_size();
    let queries = input.read_size_pair_vec(q);

    #[derive(Clone)]
    struct Node {
        value: i64,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self {
                value: i64::MIN / 2,
            }
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.value = left_val.value.max(right_val.value);
        }

        fn accumulate(&mut self, _value: &Self) {}

        fn reset_delta(&mut self) {}
    }

    let mut st = SegmentTree::<Node>::new(n);
    let mut by_vertex = vec![Vec::new(); n];
    for (i, (v, k)) in queries.enumerate() {
        by_vertex[*v - 1].push((i, *k));
    }
    let graph = Graph::from_biedges(n, &edges);
    let mut depth = vec![0; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            depth[vert].maxim(f.call(e.to(), vert) + 1);
        }
        depth[vert]
    });
    dfs.call(0, n);
    let mut ans = vec![0; q];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, level: usize| {
        for &(id, k) in &by_vertex[vert] {
            let mut res = depth[vert];
            res.maxim(st.query(level.saturating_sub(k)..level).value + level as i64);
            ans[id] = res;
        }
        let mut best = 0;
        let mut best_at = 0;
        let mut second = 0;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let cand = 1 + depth[e.to()];
            if cand > best {
                second = best;
                best = cand;
                best_at = e.to();
            } else if cand > second {
                second = cand;
            }
        }
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            st.point_update(
                level,
                Node {
                    value: if e.to() == best_at {
                        second - level as i64
                    } else {
                        best - level as i64
                    },
                },
            );
            f.call(e.to(), vert, level + 1);
        }
    });
    dfs.call(0, n, 0);
    out.print_line(ans);
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
