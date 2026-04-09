//{"name":"E - Part-Time Job Shift Management","group":"AtCoder - AtCoder Weekday Contest 0042 Beta","url":"https://atcoder.jp/contests/awc0042/tasks/awc0042_e","interactive":false,"timeLimit":2000,"tests":[{"input":"5 3 2\n3 5 2 7 4\n","output":"19\n"},{"input":"7 3 3\n10 2 8 6 1 9 3\n","output":"36\n"},{"input":"15 4 3\n12 5 8 20 3 15 7 11 9 18 6 14 2 10 16\n","output":"137\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);

    #[derive(Clone)]
    struct Node {
        value: i64,
    }
    impl Default for Node {
        fn default() -> Self {
            Self { value: i64::MAX }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.value = left_val.value.min(right_val.value);
        }
    }
    let mut st = SegmentTree::<Node>::new(n + 1);
    st.point_update(0, Node { value: 0 });
    st.point_update(1, Node { value: a[0] });
    for i in 1..n {
        let from = i.saturating_sub(k - 1);
        let to = if m != 2 { i } else { i - 1 };
        let val = st.query(from..=to).value;
        st.point_update(i + 1, Node { value: val + a[i] });
    }
    out.print_line(a.copy_sum() - st.query(n + 1 - k..=n).value);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::YesNo);

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
