use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::collections::treap::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_utils::PartialSums;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let p = input.read_size();
    let d = input.read_size();
    let a = input.read_size_vec(n);

    let t = p - d;
    #[derive(Default)]
    struct Node {
        set: TreapSet<usize>,
        multi_set: MultiTreapSet<usize>,
    }
    impl SegmentTreeNode for Node {}
    let s = a.partial_sums();
    let mut st = SegmentTree::<Node>::new(n);
    for i in 0..n {
        let x = s[i] + t;
        if let Some(pos) = s.bin_search(&x) {
            st.point_through_update(i, |node| {
                node.set.insert(pos);
                node.multi_set.insert_few(pos, pos - i);
            });
        }
    }

    for _ in 0..q {
        let l = input.read_size() - 1;
        let r = input.read_size();
        if t == 0 {
            out.print_line((0, 0));
            continue;
        }
        out.print_line(st.for_each_mut(l..r, |(qty, sum): (usize, usize), node| (qty + node.set.less_or_eq(&r), sum + node.multi_set.less_or_eq(&r))));
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
