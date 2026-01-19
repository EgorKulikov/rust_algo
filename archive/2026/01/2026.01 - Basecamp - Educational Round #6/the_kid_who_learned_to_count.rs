//{"name":"The kid who learned to count","group":"Eolymp - Basecamp - Educational Round #6","url":"https://eolymp.com/en/compete/ikoi44ho994uj2rcj49h0l45v4/problem/7","interactive":false,"timeLimit":1000,"tests":[{"input":"6 9\n1 3 1 6 6 7\n8\n1 1 6\n1 1 2\n2 4 7\n1 4 5\n1 5 6\n2 1 7\n2 3 8\n1 1 6\n","output":"Yes\nYes\nNo\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let mass = input.read_int_vec(n);

    #[derive(Default, Clone)]
    struct Node {
        min: i32,
        second_min: i32,
    }

    impl Node {
        fn new(val: i32) -> Self {
            Self {
                min: val,
                second_min: i32::MAX,
            }
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            let mut v = [
                left_val.min,
                left_val.second_min,
                right_val.min,
                right_val.second_min,
            ];
            v.sort();
            self.min = v[0];
            self.second_min = v[1];
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node::new(mass[i]));
    let m = input.read_size();
    for _ in 0..m {
        let t = input.read_int();
        match t {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let res = st.query(l..r);
                out.print_line(res.min + res.second_min <= k);
            }
            2 => {
                let pos = input.read_size() - 1;
                let val = input.read_int();
                st.point_update(pos, Node::new(val));
            }
            _ => unreachable!(),
        }
    }
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
