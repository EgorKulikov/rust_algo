//{"name":"F. Orangutan Approved Subarrays","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4 2\n1 2 2 1\n1 4\n1 3\n5 3\n1 2 1 2 1\n2 5\n3 5\n1 3\n8 4\n1 2 3 2 1 3 2 3\n1 5\n2 8\n3 5\n6 8\n","output":"YES\nYES\nNO\nYES\nYES\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FOrangutanApprovedSubarrays"}}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{Pushable, QueryResult, SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n).dec();
    let queries = input.read_size_pair_vec(q).dec();

    let mut next = vec![n; n];
    let mut next_same = vec![0; n];
    for i in (0..n).rev() {
        next_same[i] = next[a[i]];
        next[a[i]] = i;
    }

    #[derive(Default)]
    struct Node {
        variants: BTreeSet<usize>,
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
            self.variants.insert(*delta);
        }
    }

    impl QueryResult<usize, usize> for Node {
        fn empty_result(_args: &usize) -> usize {
            usize::MAX
        }

        fn result(&self, args: &usize) -> usize {
            self.variants.ceil(&args).copied().unwrap_or(usize::MAX)
        }

        fn join_results(
            left_res: usize,
            right_res: usize,
            _args: &usize,
            _left: usize,
            _mid: usize,
            _right: usize,
        ) -> usize {
            left_res.min(right_res)
        }
    }

    let mut st = SegmentTree::<Node>::new(n);
    for i in 0..n {
        st.point_through_update(i, &next_same[i]);
    }
    let mut q_at = vec![Vec::new(); n];
    for (i, (l, r)) in queries.iter().enumerate() {
        q_at[*l].push((i, *r));
    }
    let mut ans = vec![false; q];
    let mut max = n;
    for i in (0..n).rev() {
        max.minim(st.query_with_args(i + 1..next_same[i], &next_same[i]));
        for (j, r) in q_at[i].iter() {
            ans[*j] = max > *r;
        }
    }
    out.print_per_line(&ans);
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
