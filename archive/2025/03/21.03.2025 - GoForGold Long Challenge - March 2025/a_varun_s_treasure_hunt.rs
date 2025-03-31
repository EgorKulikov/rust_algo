//{"name":"A. Varun's Treasure Hunt","group":"Codeforces - GoForGold Long Challenge - March 2025","url":"https://codeforces.com/group/OseQ3LxgeG/contest/596267/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3\n2 1 -3 4 5 -6\n4\n1 4\n2 3\n1 6\n4 6\n","output":"7\n1\n11\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::bin_search::search_first_true;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);

    #[derive(Default)]
    struct Node {
        vals: Vec<i64>,
        ps: Vec<i64>,
    }

    impl Node {
        fn new(a: i64) -> Self {
            if a > 0 {
                Self {
                    vals: vec![a],
                    ps: vec![0, a],
                }
            } else {
                Self {
                    vals: vec![],
                    ps: vec![0],
                }
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.vals.extend_from_slice(&left_val.vals);
            self.vals.extend_from_slice(&right_val.vals);
            self.vals.sort_unstable();
            self.ps = self.vals.partial_sums();
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node::new(a[i]));

    let q = input.read_size();

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        let min = search_first_true(0, 1_000_000_000, |x| {
            st.for_each(l..r, |res: usize, node| res + node.vals.more(&x)) <= k
        });
        let mut sum = 0;
        let mut qty = 0;
        st.for_each(l..r, |_, node| {
            let idx = node.vals.upper_bound(&min);
            sum += node.ps[Back(0)] - node.ps[idx];
            qty += node.vals.len() - idx;
        });
        sum += min * (k - qty) as i64;
        out.print_line(sum);
    }
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
