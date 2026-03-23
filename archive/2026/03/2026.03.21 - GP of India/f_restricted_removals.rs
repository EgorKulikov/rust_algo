//{"name":"F. Restricted Removals","group":"Universal Cup - GP of India","url":"https://contest.ucup.ac/contest/3516/problem/17411","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5 2\n4 -1 7 -3 2\n4 0\n-5 -2 -1 -3\n4 4\n2 -7 5 1\n1 0\n-1\n2 1\n0 1\n3 1\n1 -1 1\n","output":"7\n0\n0\n0\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_long_vec(n);

    if k == n {
        out.print_line(0);
        return;
    }
    let s = a.partial_sums();
    let mut next = vec![0; n];
    let mut stack = vec![n];
    for i in (0..n).rev() {
        while let Some(&val) = stack.last() {
            if s[val] <= s[i] {
                stack.pop();
            } else {
                break;
            }
        }
        next[i] = stack.last().copied().unwrap_or(n);
        stack.push(i);
    }

    const INFTY: i64 = i64::MAX / 2;

    #[derive(Clone)]
    struct Node {
        val: i64,
    }
    impl Default for Node {
        fn default() -> Self {
            Self { val: i64::MAX }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val);
        }
    }

    let mut base = vec![0; n];
    let mut max = s[n];
    for i in (0..n).rev() {
        max.maxim(s[i]);
        base[i] = max - s[i];
    }
    // for i in 0..n {
    //     let j = 0;
    //     let val = base[i];
    //     debug!(j, i, val);
    // }
    let mut st = SegmentTree::with_gen(n, |i| Node { val: base[i] });
    for j in 1..n - k {
        st = SegmentTree::with_gen(n, |i| {
            if i + j >= n {
                return Node { val: INFTY };
            }
            let lim = n - j;
            let nxt = next[i].min(lim);
            let mut val = (s[nxt] - s[i]).max(0) + st.point_query(nxt).val;
            if nxt != i + 1 {
                val.minim(st.query(i + 1..nxt).val);
            }
            // debug!(j, i, val);
            Node { val }
        });
    }
    out.print_line(st.query(..).val);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");

    let mut pre_calc = ();
    // output.set_bool_output(BoolOutput::YesNo);

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
        _ => {
            unreachable!();
        }
    }
    eprint!("\x1B[0m");
    output.flush();
    input.check_empty()
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
