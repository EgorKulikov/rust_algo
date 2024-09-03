//{"name":"G2. Yunli's Subarray Queries (hard version)","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/G2","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n7 5 3\n1 2 3 2 1 2 3\n1 7\n2 7\n3 7\n8 4 2\n4 3 1 1 2 4 3 2\n3 6\n1 5\n5 4 2\n4 5 1 2 3\n1 4\n1 5\n","output":"6\n5\n2\n2\n5\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G2YunlisSubarrayQueriesHardVersion"}}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::{TaskType, TestType};
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();
    let queries = input.read_size_pair_vec(q);

    let mut ans = Vec::with_capacity(n - k + 1);
    let mut set = BTreeSet::new();
    let mut qty = vec![0; 2 * n];
    for i in 0..k - 1 {
        let val = a[i] + n - i;
        if qty[val] != 0 {
            set.remove(&(qty[val], val));
        }
        qty[val] += 1;
        set.insert((qty[val], val));
    }
    for i in k - 1..n {
        let val = a[i] + n - i;
        if qty[val] != 0 {
            set.remove(&(qty[val], val));
        }
        qty[val] += 1;
        set.insert((qty[val], val));
        ans.push(k - set.iter().next_back().unwrap().0);
        let val = a[i - (k - 1)] + n - (i - (k - 1));
        set.remove(&(qty[val], val));
        qty[val] -= 1;
        if qty[val] != 0 {
            set.insert((qty[val], val));
        }
    }
    let mut res = vec![0; q];
    let mut qq = vec![Vec::new(); n - k + 1];
    for (i, &(l, r)) in queries.iter().enumerate() {
        qq[l - 1].push((i, r));
    }

    #[derive(Clone, Default)]
    struct Node {
        sum: usize,
        delta: Option<usize>,
    }

    impl SegmentTreeNode for Node {
        fn new(_left: usize, _right: usize) -> Self {
            Self::default()
        }

        fn join(
            &mut self,
            left_val: &Self,
            right_val: &Self,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) {
            self.sum = left_val.sum + right_val.sum;
        }

        fn accumulate(&mut self, value: &Self, left: usize, right: usize) {
            if let Some(delta) = value.delta {
                self.sum = delta * (right - left);
                self.delta = Some(delta);
            }
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {
            self.delta = None;
        }
    }

    let mut st = SegmentTree::<Node>::new(n - k + 1);
    let mut mins = Vec::new();
    for i in (0..n - k + 1).rev() {
        while let Some(&j) = mins.last() {
            if ans[j] >= ans[i] {
                mins.pop();
            } else {
                break;
            }
        }
        if let Some(&j) = mins.last() {
            st.update(
                i..j,
                &Node {
                    sum: 0,
                    delta: Some(ans[i]),
                },
            );
        } else {
            st.update(
                i..,
                &Node {
                    sum: 0,
                    delta: Some(ans[i]),
                },
            );
        }
        mins.push(i);
        for &(id, r) in &qq[i] {
            res[id] = st.query::<Node>(i..=r - k).sum;
        }
    }
    out.print_per_line(&res);
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
