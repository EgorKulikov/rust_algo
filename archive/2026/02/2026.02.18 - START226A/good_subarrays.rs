//{"name":"Good Subarrays","group":"CodeChef - START226A","url":"https://www.codechef.com/START226A/problems/GOODSUB6","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 2\n4\n3 1 2 4\n7\n1 1 2 1 2 3 4\n2\n2 2\n","output":"3\n2\n17\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n).dec();

    #[derive(Default, Clone)]
    struct Node {
        val: usize,
        delta: Option<usize>,
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val);
        }

        fn accumulate(&mut self, value: &Self) {
            if let Some(val) = value.delta {
                self.val = val;
                self.delta = Some(val);
            }
        }

        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }

    let mut st = SegmentTree::<Node>::new(n);
    st.update(
        ..,
        &Node {
            delta: Some(n),
            ..Default::default()
        },
    );

    let mut ans = 0;
    for i in (0..n).rev() {
        st.update(
            ..a[i],
            &Node {
                delta: Some(i),
                ..Default::default()
            },
        );
        st.update(
            a[i]..=a[i],
            &Node {
                delta: Some(n),
                ..Default::default()
            },
        );
        ans += st.query(..).val - i;
    }
    out.print_line(ans);
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
