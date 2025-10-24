//{"name":"C. Limited Edition Shop","group":"Codeforces - Codeforces Round 1053 (Div. 1)","url":"https://codeforces.com/contest/2150/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3\n1 -1 1\n3 1 2\n2 3 1\n3\n-2 5 2\n3 1 2\n2 3 1\n3\n-1 -2 -3\n3 1 2\n2 3 1\n3\n1000000000 1000000000 1000000000\n3 1 2\n2 3 1\n4\n5 -15 10 -5\n2 4 3 1\n1 4 2 3\n4\n-5 -5 -5 100\n2 3 1 4\n4 1 2 3\n4\n-1 -100 5 10\n1 2 3 4\n2 3 4 1\n12\n-4 6 10 10 1 -8 6 2 -8 -4 0 -6\n11 12 7 3 6 8 1 5 10 2 9 4\n7 5 3 6 1 2 8 12 9 4 10 11\n","output":"2\n5\n0\n3000000000\n10\n85\n14\n24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::permutation::Permutation;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let v = input.read_long_vec(n);
    let a = input.read_size_vec(n).dec();
    let b = input.read_size_vec(n).dec();

    let b_inv = b.inv();
    struct Node {
        val: i64,
        add_delta: i64,
        max_delta: i64,
    }

    impl Default for Node {
        fn default() -> Self {
            Self {
                val: 0,
                add_delta: 0,
                max_delta: i64::MIN / 2,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.max(right_val.val);
        }

        fn accumulate(&mut self, value: &Self) {
            self.val += value.add_delta;
            self.add_delta += value.add_delta;
            self.max_delta += value.add_delta;
            self.val.maxim(value.max_delta);
            self.max_delta.maxim(value.max_delta);
        }

        fn reset_delta(&mut self) {
            self.add_delta = 0;
            self.max_delta = i64::MIN / 2;
        }
    }

    let mut st = SegmentTree::<Node>::new(n + 1);
    for i in 0..n {
        st.update(
            ..=b_inv[a[i]],
            &Node {
                add_delta: v[a[i]],
                ..Default::default()
            },
        );
        let max = st.point_query(b_inv[a[i]]).val;
        st.update(
            b_inv[a[i]] + 1..,
            &Node {
                max_delta: max,
                ..Default::default()
            },
        );
    }
    out.print_line(st.point_query(n).val);
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
