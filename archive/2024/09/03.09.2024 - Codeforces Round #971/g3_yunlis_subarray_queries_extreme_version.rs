//{"name":"G3. Yunli's Subarray Queries (extreme version)","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/G3","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n7 2 4\n1 2 3 2 1 2 3\n4 6\n1 7\n2 7\n3 7\n8 4 2\n4 3 1 1 2 4 3 2\n3 6\n1 5\n5 4 2\n4 5 1 2 3\n1 4\n1 5\n10 4 8\n2 3 6 5 8 9 8 10 10 1\n2 7\n6 10\n1 9\n1 6\n3 9\n4 10\n2 10\n1 8\n10 7 4\n3 4 5 3 4 5 9 10 8 9\n1 9\n2 10\n1 10\n2 9\n","output":"1\n3\n3\n3\n2\n7\n2\n4\n8\n6\n28\n7\n16\n20\n32\n19\n18\n15\n26\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G3YunlisSubarrayQueriesExtremeVersion"}}}

use algo_lib::collections::segment_tree::{Pushable, SegmentTree, SegmentTreeNode};
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
        per_add: usize,
        add_single: Option<usize>,
        delta: usize,
        lazy_adds: usize,
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
            self.per_add = left_val.per_add + right_val.per_add;
        }

        fn accumulate(&mut self, value: &Self, left: usize, right: usize) {
            self.sum += self.per_add * value.lazy_adds;
            self.sum += value.delta * (right - left);
            self.delta += value.delta;
            if let Some(add_single) = self.add_single {
                self.delta += value.lazy_adds * add_single;
            } else {
                self.lazy_adds += value.lazy_adds;
            }
            if let Some(add_single) = value.add_single {
                self.per_add = add_single * (right - left);
                self.add_single = Some(add_single);
            }
        }

        fn reset_delta(&mut self, _left: usize, _right: usize) {
            self.delta = 0;
            self.lazy_adds = 0;
            self.add_single = None;
        }
    }

    impl Pushable<&Option<usize>> for Node {
        fn push(&mut self, delta: &Option<usize>, left: usize, right: usize) {
            if let Some(delta) = *delta {
                self.add_single = Some(delta);
                self.per_add = (right - left) * delta;
            }
            self.sum += self.per_add;
            if let Some(add_single) = self.add_single {
                self.delta += add_single;
            } else {
                self.lazy_adds += 1;
            }
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
            st.update(i..j, &Some(ans[i]));
            st.update(j.., &None);
        } else {
            st.update(i.., &Some(ans[i]));
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
