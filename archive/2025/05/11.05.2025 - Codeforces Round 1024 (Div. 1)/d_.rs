//{"name":"D. Мани и подпоследовательности","group":"Codeforces - Codeforces Round 1024 (Div. 1)","url":"https://codeforces.com/contest/2101/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3\n3 1 2\n5\n2 3 4 5 1\n4\n3 4 1 2\n7\n1 2 3 4 5 6 7\n10\n7 8 2 4 5 10 1 3 6 9\n","output":"6\n15\n9\n28\n36\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    let mut from = vec![0; n];
    let mut to = vec![n; n];
    let mut order = (0..n).collect::<Vec<_>>().sorted_by_key(|i| a[*i]);
    for _ in 0..2 {
        let mut set = BTreeSet::new();
        #[derive(Clone)]
        struct MinNode {
            val: usize,
        }
        impl Default for MinNode {
            fn default() -> Self {
                MinNode { val: usize::MAX }
            }
        }
        impl SegmentTreeNode for MinNode {
            fn update(&mut self, left_val: &Self, right_val: &Self) {
                self.val = left_val.val.min(right_val.val);
            }
        }
        #[derive(Default, Clone)]
        struct MaxNode {
            val: usize,
        }
        impl SegmentTreeNode for MaxNode {
            fn update(&mut self, left_val: &Self, right_val: &Self) {
                self.val = left_val.val.max(right_val.val);
            }
        }
        let mut st_from = SegmentTree::<MaxNode>::new(n);
        let mut st_to = SegmentTree::<MinNode>::new(n);
        for i in order.copy_iter() {
            from[i].maxim(st_from.query(..i).val);
            to[i].minim(st_to.query(i..).val);
            if let Some(&j) = set.prev(&i) {
                st_to.point_update(j, MinNode { val: i });
            }
            if let Some(&j) = set.next(&i) {
                st_from.point_update(j, MaxNode { val: i + 1 });
            }
            set.insert(i);
        }
        order.reverse();
    }
    let mut pairs = Vec::new();
    for i in 0..n {
        pairs.push((from[i], to[i]));
    }
    pairs.sort();
    let mut end = 0;
    let mut ans = 0;
    for (from, to) in pairs {
        if to > end {
            if from < end {
                ans += (end - from) * (to - end);
            }
            ans += (to - end) * (to - end + 1) / 2;
            end = to;
        }
    }
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
