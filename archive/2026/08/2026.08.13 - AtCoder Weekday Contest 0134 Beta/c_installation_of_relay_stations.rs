use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_int();
    let xc = input.read_int_pair_vec(n).sorted();

    #[derive(Clone)]
    struct Node {
        val: i32,
    }
    impl Default for Node {
        fn default() -> Self {
            Self { val: i32::MAX }
        }
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.min(right_val.val);
        }
    }
    let mut st = SegmentTree::with_gen(n, |i| Node { val: xc[i].1 });
    let mut start = 0;
    let mut ans = None;
    for i in 0..n {
        while xc[i].0 - xc[start].0 > d {
            start += 1;
        }
        if start != i {
            ans.minim(xc[i].1 + st.query(start..i).val);
        }
    }
    out.print_line(ans);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
