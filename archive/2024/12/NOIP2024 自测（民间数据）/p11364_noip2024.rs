//{"name":"P11364 [NOIP2024] 树上查询（民间数据）","group":"Luogu","url":"https://www.luogu.com.cn/problem/P11364?contestId=217331","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5 6\n6 1\n6 2\n2 3\n2 4\n3\n2 5 2\n1 4 1\n1 6 3\n","output":"3\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"P11364NOIP2024"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
use algo_lib::graph::lca::LCATrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::HashSet;
use std::iter::once;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).do_with(|v| {
        for (a, b) in v.iter_mut() {
            *a -= 1;
            *b -= 1;
        }
    });
    let q = input.read_size();
    let queries = input.read_vec::<(usize, usize, usize)>(q);

    let mut by_k = vec![Vec::new(); n + 1];
    for (i, (l, r, k)) in queries.into_iter().enumerate() {
        by_k[k].push((i, l - 1, r - 1));
    }
    let graph = Graph::from_biedges(n, &edges);
    let lca = graph.lca();
    let mut ans = vec![0; q];
    #[derive(Default, Clone)]
    struct Node {
        max: i32,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(&mut self, left_val: &Self, right_val: &Self) {
            self.max = left_val.max.max(right_val.max);
        }
    }

    impl Pushable<()> for Node {
        fn push(&mut self, _delta: ()) {
            self.max = 0;
        }
    }

    let mut st = SegmentTree::from_generator(n, |i| Node {
        // len: 1,
        max: lca.level(i) as i32,
    });

    for (i, l, r) in by_k[1].iter().copied() {
        ans[i] = st.query(l..=r).max + 1;
    }

    let mut prev = once(n).chain(0..n - 2).collect::<Vec<_>>();
    let mut next = (1..n - 1).chain(once(n)).collect::<Vec<_>>();
    let mut vals = Vec::with_capacity(n - 1);
    for i in 0..n - 1 {
        vals.push(lca.level(lca.lca(i, i + 1)));
    }
    let mut st = SegmentTree::from_generator(n - 1, |i| Node {
        max: vals[i] as i32,
    });
    struct FT {
        val: Vec<i32>,
    }
    impl FT {
        fn new(n: usize) -> Self {
            Self { val: vec![0; n] }
        }
        fn add(&mut self, mut x: usize, delta: i32) {
            let len = self.val.len();
            while x < len {
                self.val[x] += delta;
                x |= x + 1;
            }
        }
        fn query(&self, mut x: usize) -> i64 {
            let mut res = 0;
            while x > 0 {
                x -= 1;
                res += self.val[x];
                x &= x + 1;
            }
            res as i64
        }
    }
    let mut ft = FT::new(n - 1);
    for i in 0..n - 1 {
        ft.add(i, 1);
    }
    let mut first_max = n;
    let mut last_max = n;
    let mut prev_max = vec![n; n - 1];
    let mut next_max = vec![n; n - 1];
    // let mut maximums = HashSet::new();
    let mut minimums = Vec::new();
    let mut is_maximum = vec![false; n - 1];
    for i in 0..n - 1 {
        if i > 0 && i < n - 2 && vals[i] < vals[i - 1] && vals[i] <= vals[i + 1] {
            if first_max == n {
                first_max = i;
            }
            if last_max != n {
                next_max[last_max] = i;
                prev_max[i] = last_max;
            }
            last_max = i;
            is_maximum[i] = true;
        }
        if (i == 0 || vals[i] >= vals[i - 1]) && (i == n - 2 || vals[i] > vals[i + 1]) {
            minimums.push(i);
        }
    }
    let mut len = vec![1; n - 1];

    let mut total = 0;
    for k in 2..=n {
        assert_eq!(ft.query(n - 1) as usize, n + 1 - k);
        for (i, l, r) in by_k[k].iter().copied() {
            let find = |mut x: usize| {
                let mut left = 0;
                let mut right = n - 1;
                for i in (0..20).rev() {
                    if (left >> i & 1) == 0 && right >> i & 1 == 1 {
                        let mid = left + (1 << i);
                        if ft.val[mid - 1] > x as i32 {
                            right = mid - 1;
                        } else {
                            x -= ft.val[mid - 1] as usize;
                            left = mid;
                        }
                    }
                }
                assert_ne!(left, n - 1);
                left
            };
            let left = find(l);
            let right = find(r + 1 - k);
            ans[i] = st.query(left..=right).max + 1;
        }
        if k != n {
            let mut i = first_max;
            while i != n {
                total += 1;
                ft.add(i, 1);
                len[i] += 1;
                i = next_max[i];
            }
            let mut poi = HashSet::new();
            for i in minimums.drain(..) {
                total += 1;
                ft.add(i, -1);
                len[i] -= 1;
                if len[i] == 0 {
                    st.point_update(i, ());
                    let left = prev[i];
                    if left != n {
                        poi.insert(left);
                        next[left] = next[i];
                    }
                    let right = next[i];
                    if right != n {
                        poi.insert(right);
                        prev[right] = prev[i];
                    }
                } else {
                    poi.insert(i);
                }
            }
            for i in poi {
                let left = prev[i];
                let right = next[i];
                if is_maximum[i]
                    && (left == n || right == n || vals[i] >= vals[left] || vals[i] > vals[right])
                {
                    if first_max == i {
                        first_max = next_max[i];
                    }
                    if prev_max[i] != n {
                        next_max[prev_max[i]] = next_max[i];
                    }
                    if next_max[i] != n {
                        prev_max[next_max[i]] = prev_max[i];
                    }
                    is_maximum[i] = false;
                }
                if (left == n || vals[i] >= vals[left]) && (right == n || vals[i] > vals[right]) {
                    minimums.push(i);
                }
            }
        }
    }
    out.print_per_line(&ans);
    eprintln!("Total: {}", total);
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
