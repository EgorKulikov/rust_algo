use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let v = input.read_long_vec(n);
    let lr = input.read_size_pair_vec(m).dec();

    let s = v.partial_sums();
    #[derive(Default, Clone)]
    struct Node {
        val: i64,
        val_r: i64,
    }
    impl SegmentTreeNode for Node {
        fn update(&mut self, left_val: &Self, right_val: &Self) {
            self.val = left_val.val.max(right_val.val);
            self.val_r = left_val.val_r.max(right_val.val_r);
        }
    }

    let mut res = vec![0; n + 1];
    for _ in 0..k {
        let mut st = SegmentTree::with_gen(n + 1, |i| Node {
            val: res[i],
            val_r: res[i] - s[i],
        });
        res.fill(0);
        for (l, r) in lr.copy_iter() {
            res[r + 1].maxim(st.query(..l).val + s[r + 1] - s[l]);
            res[r + 1].maxim(st.query(l..=r).val_r + s[r + 1]);
        }
    }
    out.print_line(res.copy_max());
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
