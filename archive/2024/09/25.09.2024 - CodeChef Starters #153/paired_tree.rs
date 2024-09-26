//{"name":"Paired Tree","group":"CodeChef - START153A","url":"https://www.codechef.com/START153A/problems/PTRE","interactive":false,"timeLimit":2500,"tests":[{"input":"2\n2 3\n1 2\n2 3\n3 4\n1 2 2 1\n2 2\n1 2 4\n2 1\n4 6\n1 2\n3 1\n6 7\n5 2\n6 2\n8 6\n4 2\n3 4 2 4 1 3 1 2\n2 1\n1 6 3\n2 1\n1 4 5\n2 4\n2 1\n","output":"0\n1\n0\n1\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PairedTree"}}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::hl_decomposition::HLDecomposition;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::cell::Cell;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let edges = input.read_size_pair_vec(2 * n - 1).dec();
    let mut a = input.read_size_vec(2 * n).dec();

    let graph = Graph::from_biedges(2 * n, &edges);
    let mut wh = vec![Vec::with_capacity(2); n];
    for i in 0..2 * n {
        wh[a[i]].push(i);
    }

    let lca = graph.lca();
    let (paths, id, pos) = graph.hl_decomposition();
    #[derive(Default)]
    struct Node {
        val: i32,
    }
    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Default::default()
        }

        fn join(
            &mut self,
            _left_val: &Self,
            _right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
        }

        fn accumulate(&mut self, _value: &Self, _left: usize, _right: usize) {}

        fn reset_delta(&mut self, _left: usize, _right: usize) {}
    }
    impl Pushable<&i32> for Node {
        fn push(&mut self, delta: &i32, _: usize, _: usize) {
            self.val += delta;
        }
    }
    impl Pushable<&Cell<i32>> for Node {
        fn push(&mut self, delta: &Cell<i32>, _: usize, _: usize) {
            delta.set(delta.get() + self.val);
        }
    }
    let mut path_trees = Vec::with_capacity(paths.len());
    let mut lca_fts = Vec::with_capacity(paths.len());
    for i in paths.indices() {
        path_trees.push(SegmentTree::<Node>::new(paths[i].len()));
        lca_fts.push(FenwickTree::new(paths[i].len()));
    }
    for i in 0..n {
        let l = lca.lca(wh[i][0], wh[i][1]);
        lca_fts[id[l]].add(pos[l], 1);
        for mut v in wh[i].iter().copied() {
            while v != l {
                if id[v] != id[l] {
                    path_trees[id[v]].update(..=pos[v], &1);
                    v = lca.parent(paths[id[v]][0]).unwrap();
                } else {
                    path_trees[id[v]].update(pos[l] + 1..=pos[v], &1);
                    break;
                }
            }
        }
        path_trees[id[l]].update(pos[l]..=pos[l], &1);
    }

    for _ in 0..q {
        match input.read_int() {
            1 => {
                let u = input.read_size() - 1;
                let v = input.read_size() - 1;
                if a[u] == a[v] {
                    continue;
                }
                for x in [u, v] {
                    let i = a[x];
                    let l = lca.lca(wh[i][0], wh[i][1]);
                    lca_fts[id[l]].add(pos[l], -1);
                    for mut v in wh[i].iter().copied() {
                        while v != l {
                            if id[v] != id[l] {
                                path_trees[id[v]].update(..=pos[v], &-1);
                                v = lca.parent(paths[id[v]][0]).unwrap();
                            } else {
                                path_trees[id[v]].update(pos[l] + 1..=pos[v], &-1);
                                break;
                            }
                        }
                    }
                    path_trees[id[l]].update(pos[l]..=pos[l], &-1);
                }
                a.swap(u, v);
                for x in [u, v] {
                    let i = a[x];
                    for j in &mut wh[i] {
                        if *j == u + v - x {
                            *j = x;
                            break;
                        }
                    }
                    let l = lca.lca(wh[i][0], wh[i][1]);
                    lca_fts[id[l]].add(pos[l], 1);
                    for mut v in wh[i].iter().copied() {
                        while v != l {
                            if id[v] != id[l] {
                                path_trees[id[v]].update(..=pos[v], &1);
                                v = lca.parent(paths[id[v]][0]).unwrap();
                            } else {
                                path_trees[id[v]].update(pos[l] + 1..=pos[v], &1);
                                break;
                            }
                        }
                    }
                    path_trees[id[l]].update(pos[l]..=pos[l], &1);
                }
            }
            2 => {
                let k = input.read_size() - 1;
                let cell = Cell::new(0);
                let l = lca.lca(wh[k][0], wh[k][1]);
                path_trees[id[l]].point_through_update(pos[l], &cell);
                let mut ans = cell.get();
                for mut v in wh[k].iter().copied() {
                    while v != l {
                        if id[v] != id[l] {
                            ans += lca_fts[id[v]].get(..=pos[v]);
                            v = lca.parent(paths[id[v]][0]).unwrap();
                        } else {
                            ans += lca_fts[id[v]].get(pos[l] + 1..=pos[v]);
                            break;
                        }
                    }
                }
                out.print_line((n as i32) - ans);
            }
            _ => unreachable!(),
        }
    }
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
        TaskType::Classic => {
            input.skip_whitespace();
            input.peek().is_none()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
