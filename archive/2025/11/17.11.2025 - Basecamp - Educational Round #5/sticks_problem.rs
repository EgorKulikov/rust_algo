//{"name":"Sticks Problem","group":"Eolymp - Basecamp - Educational Round #5","url":"https://eolymp.com/en/compete/saaq21a9v514tbj0spj7bmgjpk/problem/8","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 4 3 6\n4\n6 5 4 3\n9\n12 4 8 7 5 9 6 3 1\n","output":"1\n-1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_int_vec(n);

    #[derive(Copy, Clone)]
    struct Node {
        min: i32,
        max: i32,
    }
    impl Default for Node {
        fn default() -> Self {
            Self {
                min: i32::MAX,
                max: i32::MIN,
            }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.min = left_val.min.min(right_val.min);
            self.max = left_val.max.max(right_val.max);
        }
    }
    let mut st = SegmentTree::<Node>::new(n);
    let mut ans = None;
    for i in 0..n {
        let p = st
            .binary_search_in_rtl(..i, |node| node.max >= s[i], |_, pos| pos + 1)
            .unwrap_or(0);
        if p != i {
            let min = st.query(p..i).min;
            let q = st
                .binary_search_in_rtl(p..i, |node| node.min == min, |_, pos| pos)
                .unwrap();
            ans.maxim(i - q);
        }
        st.point_through_update(i, |node| {
            node.min.minim(s[i]);
            node.max.maxim(s[i]);
        });
    }
    out.print_line(ans);
}

pub static TEST_TYPE: TestType = TestType::MultiEof;
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
