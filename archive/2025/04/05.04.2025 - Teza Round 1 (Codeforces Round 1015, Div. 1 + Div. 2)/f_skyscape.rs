//{"name":"F. Skyscape","group":"Codeforces - Teza Round 1 (Codeforces Round 1015, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2084/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"9\n2\n2 1\n1 2\n4\n3 2 4 1\n2 0 0 1\n5\n3 2 1 5 4\n1 3 0 0 0\n5\n3 2 1 5 4\n3 2 1 5 4\n5\n3 2 1 5 4\n3 2 5 1 4\n6\n3 5 6 2 1 4\n0 2 0 5 0 0\n6\n3 5 6 2 1 4\n0 2 0 6 4 0\n9\n6 9 2 4 1 7 8 3 5\n0 2 5 9 0 0 0 8 0\n9\n8 5 3 9 1 7 4 6 2\n0 0 8 0 7 0 4 0 2\n","output":"1 2\n2 3 4 1\n1 3 2 4 5\n3 2 1 5 4\n-1\n3 2 1 5 4 6\n-1\n-1\n1 3 8 5 7 9 4 6 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let mut c = input.read_size_vec(n);

    #[derive(Clone, Default)]
    struct MaxNode {
        val: usize,
    }
    impl SegmentTreeNode for MaxNode {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.max(right_val.val);
        }
    }

    let mut st = SegmentTree::<MaxNode>::new(n + 1);
    let mut st_min = SegmentTree::<Node>::new(n + 1);

    let mut c_inv = vec![n; n + 1];
    for i in 0..n {
        if c[i] != 0 {
            c_inv[c[i]] = i;
        }
    }
    // let mut missing = BTreeSet::new();
    let mut min_pos = vec![0; n + 1];
    // let mut graph = Graph::new(n + 1);
    // let mut in_deg = vec![0; n + 1];
    for i in 0..n {
        if c_inv[a[i]] == n {
            min_pos[a[i]] = st.query(1..a[i]).val;
            // if let Some(&prev) = missing.prev(&a[i]) {
            //     graph.add_edge(Edge::new(prev, a[i]));
            //     in_deg[a[i]] += 1;
            // }
            // missing.insert(a[i]);
        } else {
            st.point_update(a[i], MaxNode { val: c_inv[a[i]] });
        }
    }
    let mut max_pos = vec![0; n + 1];
    for i in (0..n).rev() {
        if c_inv[a[i]] == n {
            max_pos[a[i]] = st_min.query(a[i]..).val;
        } else {
            st_min.point_update(a[i], Node { val: c_inv[a[i]] });
        }
    }
    let mut roots = BTreeSet::new();
    let mut available = vec![Vec::new(); n];
    for i in 0..n {
        if c_inv[a[i]] == n {
            if min_pos[a[i]] == 0 {
                roots.insert((max_pos[a[i]], a[i]));
            }
        }
        if min_pos[a[i]] != 0 {
            available[min_pos[a[i]]].push(a[i]);
        }
    }
    for i in 0..n {
        if c[i] == 0 {
            if roots.is_empty() {
                out.print_line(-1);
                return;
            }
            c[i] = roots.pop_first().unwrap().1;
        } else {
            for x in available[i].copy_iter() {
                roots.insert((max_pos[x], x));
            }
        }
    }
    // let mut missing = BitSet::new(n + 1);
    // missing.fill(true);
    // missing.unset(0);
    // for i in 0..n {
    //     if c[i] != 0 {
    //         missing.unset(c[i]);
    //     }
    // }
    // let mut x = Vec::new();
    // for i in 0..n {
    //     if missing[a[i] + 1] {
    //         x.push(a[i] + 1);
    //     }
    // }
    // let mut at = 0;
    // for i in 0..n {
    //     if c[i] == 0 {
    //         c[i] = x[at];
    //         at += 1;
    //     }
    // }
    let a = a.dec();
    let c = c.dec();
    let a_inv = a.inv();
    let c_inv = c.inv();
    #[derive(Clone)]
    struct Node {
        val: usize,
    }

    impl Default for Node {
        fn default() -> Self {
            Self { val: usize::MAX }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val);
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node { val: c_inv[a[i]] });

    for i in 0..n {
        let leftmost = st.query(a_inv[i] + 1..);
        if leftmost.val < c_inv[i] {
            out.print_line(-1);
            return;
        }
        st.point_update(a_inv[i], Node::default());
    }
    out.print_line(c.inc());
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
