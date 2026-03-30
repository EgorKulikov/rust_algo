//{"name":"Bubble XOR Sort","group":"Eolymp - Basecamp - Weekend Practice #21 ","url":"https://eolymp.com/en/compete/idp1eoul596rpbbmp0mfbbo94g/problem/3","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3\n1 1 2\n1 2 1\n5\n1 2 3 4 5\n5 4 3 2 1\n9\n1 2 1 9 9 4 3 2 8\n8 9 2 4 1 1 2 9 3\n","output":"3\n42\n128\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::default_map::by_index;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    let b = input.read_size_vec(n);

    let mut pos = by_index(&a);
    for v in pos.values_mut() {
        v.reverse();
    }

    #[derive(Default, Clone)]
    struct Node {
        zeroes: [usize; 20],
        ones: [usize; 20],
    }

    impl Node {
        fn new(val: usize) -> Self {
            let mut res = Self::default();
            for i in 0..20 {
                if !val.is_set(i) {
                    res.zeroes[i] = 1;
                } else {
                    res.ones[i] = 1;
                }
            }
            res
        }
    }

    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            for i in 0..20 {
                self.zeroes[i] = left_val.zeroes[i] + right_val.zeroes[i];
                self.ones[i] = left_val.ones[i] + right_val.ones[i];
            }
        }
    }

    let mut st = SegmentTree::with_gen(n, |i| Node::new(a[i]));
    let mut ans = 0;
    for i in 0..n {
        let pos = pos[b[i]].pop().unwrap();
        let res = st.query(..pos);
        for j in 0..20 {
            if b[i].is_set(j) {
                ans += res.zeroes[j] << j;
            } else {
                ans += res.ones[j] << j;
            }
        }
        st.point_update(pos, Node::default());
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
