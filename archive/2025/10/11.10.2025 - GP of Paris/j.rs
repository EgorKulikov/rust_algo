//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fenwick::FenwickTree;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let u = input.read_size();
    let r = input.read_long();
    let mut s = input.read_long_vec(n);
    let q = input.read_size();

    if u + 1 >= n {
        let mut ans = s.copy_sum();
        out.print_line(ans);
        for _ in 0..q {
            let at = input.read_size() - 1;
            let val = input.read_long();
            ans += val - s[at];
            s[at] = val;
            out.print_line(ans);
        }
        return;
    }
    struct Node {
        val: i64,
        delta: i64,
    }

    impl Default for Node {
        fn default() -> Self {
            Self {
                val: i64::MAX,
                delta: 0,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val);
        }

        fn accumulate(&mut self, value: &Self) {
            self.val += value.delta;
            self.delta += value.delta;
        }

        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }

    let ps = s.partial_sums();
    let mut st = SegmentTree::with_gen(n - u - 1, |i| Node {
        val: ps[i + u + 1] - ps[i + 1],
        ..Default::default()
    });

    let mut ft = FenwickTree::from(s.as_slice());
    for id in 0.. {
        let pos = st
            .binary_search_in(0..n - u - 1, |node| node.val <= r, |_, pos| pos + u + 1)
            .unwrap_or(n);
        out.print_line(ft.get(..pos));
        if id == q {
            break;
        }
        let at = input.read_size() - 1;
        let val = input.read_long();
        let delta = val - s[at];
        s[at] = val;
        ft.add(at, delta);
        if at != 0 {
            st.update(at.saturating_sub(u)..at, &Node { val: 0, delta });
        }
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
