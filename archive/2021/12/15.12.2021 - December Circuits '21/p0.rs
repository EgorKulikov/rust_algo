//{"name":"Task","group":"HackerEarth - December Circuits '21","url":"https://www.hackerearth.com/challenges/competitive/december-circuits-21-2/algorithm/path-smallest-63a01482/","interactive":false,"timeLimit":1000,"tests":[{"input":"8 2\n105 2 9 3 8 5 7 7\n1 2\n1 3\n1 4\n3 5\n3 6\n3 7\n4 8\n8 7 5\n8 7 3\n","output":"105 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Task"}}}
use algo_lib::collections::arr2d::Arr2d;
use algo_lib::collections::iter_ext::IterExt;
use algo_lib::collections::segment_tree::{OperationClosure, SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::{Bounds, IncDec};
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::graph::hl_decomposition::HLDecomposition;
use algo_lib::graph::lca::LCATrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::{out, out_line, transparent_wrapper};
use std::ops::{Deref, DerefMut};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let q: usize = input.read();
    let a: Vec<u32> = input.read_vec(n);
    let edges = input.read_vec::<(usize, usize)>(n - 1).dec_by_one();
    let queries = input.read_vec::<(usize, usize, usize)>(q).dec_by_one();

    let mut graph = Graph::new(n);
    for (u, v) in edges {
        graph.add_edge(u, BiEdge::new(v));
    }
    let lca = graph.lca();
    /*let (paths, id, pos) = graph.hl_decomposition();
    transparent_wrapper!(Node, Vec<u32>);

    impl SegmentTreeNode for Node {
        fn new(left: usize, right: usize) -> Self {
            Node(vec![0u32; right - left])
        }

        fn join(&mut self, left: &Self, right: &Self) {
            self[..left.len()].copy_from_slice(&left);
            self[left.len()..].copy_from_slice(&right);
            self.sort_unstable();
        }

        fn accumulate(&mut self, value: &Self) {
            if value.len() == 1 {
                self[0] = value[0];
            }
        }

        fn reset_delta(&mut self) {}
    }
    enum Tree {
        SegmentTree(SegmentTree<Node>),
        Small(Arr2d<Vec<u32>>),
    }

    impl Tree {
        pub fn new(path: &Vec<usize>, a: &Vec<u32>) -> Self {
            if path.len() <= 40000 {
                let mut res = Arr2d::new(path.len(), path.len(), Vec::new());
                for i in 0..path.len() {
                    for j in i..path.len() {
                        res[(i, j)] = path[i..(j + 1)].iter().map(|i| a[*i]).collect_vec();
                        res[(i, j)].sort_unstable();
                    }
                }
                Tree::Small(res)
            } else {
                Tree::SegmentTree(SegmentTree::from_generator(path.len(), |at| {
                    Node(vec![a[path[at]]])
                }))
            }
        }

        pub fn operation(
            &mut self,
            from: usize,
            to: usize,
            tree_op: &mut OperationClosure<Node, u32, usize>,
            v: &u32,
        ) -> usize {
            match self {
                Tree::SegmentTree(tree) => tree.operation(from, to, tree_op, v),
                Tree::Small(inner) => inner[(from, to - 1)].as_slice().upper_bound(v),
            }
        }
    }
    let mut trees = Vec::new();
    for path in paths.iter() {
        trees.push(Tree::new(path, &a));
    }
    let mut tree_op: OperationClosure<Node, u32, usize> = OperationClosure::new(
        |node: &mut Node, up: &u32| node.as_slice().upper_bound(up),
        |left, right, _| left + right,
        |_, _, _| 0,
    );
    let mut get = |mut u: usize, /*l: usize, */ v: u32| -> usize {
        let mut res = 0;
        loop {
            if id[u] == 0 {
                res += trees[id[u]].operation(0, pos[u] + 1, &mut tree_op, &v);
                break;
            } else {
                res += trees[id[u]].operation(0, pos[u] + 1, &mut tree_op, &v);
                u = lca.parent(paths[id[u]][0]).unwrap();
            }
        }
        res
    };*/
    let mut paths = vec![Vec::new(); n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        if vert != prev {
            paths[vert] = paths[prev].clone();
        }
        paths[vert].push(a[vert]);
        for i in (1..paths[vert].len()).rev() {
            if paths[vert][i] < paths[vert][i - 1] {
                paths[vert].swap(i, i - 1);
            } else {
                break;
            }
        }
        for e in graph[vert].iter() {
            if e.to() != prev {
                f.call(e.to(), vert);
            }
        }
    });
    dfs.call(0, 0);
    let get = |vert: usize, val: u32| paths[vert].as_slice().upper_bound(&val);
    let mut b = a.clone();
    b.sort_unstable();
    b.dedup();
    let mut ans = Vec::new();
    for (u, v, k) in queries {
        let path_length = lca.path_length(u, v);
        if path_length + 1 <= k {
            ans.push(-1);
            continue;
        }
        let l = lca.lca(u, v);
        let mut left = 0;
        let mut right = n;
        while left < right {
            let mid = (left + right) >> 1;
            let qty = get(u, b[mid]) + get(v, b[mid]) - 2 * get(l, b[mid])
                + if a[l] <= b[mid] { 1 } else { 0 };
            if qty <= k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans.push(b[left] as i32);
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
