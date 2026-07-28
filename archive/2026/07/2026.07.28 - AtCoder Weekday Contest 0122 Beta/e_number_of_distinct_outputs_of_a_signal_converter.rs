use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::treap::multi_treap_set::MultiTreapSet;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let q = input.read_size();
    let lrx = input.read_vec::<(usize, usize, usize)>(n);

    #[derive(Default)]
    struct Node {
        val: usize,
        delta: Option<usize>,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            if let Some(delta) = value.delta {
                self.val = delta;
                self.delta = Some(delta);
            }
        }
        fn reset_delta(&mut self) {
            self.delta = None;
        }
    }
    let mut st = SegmentTree::with_gen(k + 1, |i| Node { val: i, delta: None });
    for (l, r, x) in lrx.copy_rev() {
        let val = st.point_query(x).val;
        st.update(l..=r, &Node { delta: Some(val), val: 0 });
    }
    let res = Vec::with_gen(k, |i| st.point_query(i + 1).val);
    let mut first = vec![k; k + 1];
    let mut next = vec![0; k];
    for i in (0..k).rev() {
        next[i] = first[res[i]];
        first[res[i]] = i;
    }

    #[derive(Default)]
    struct Node2 {
        tree: MultiTreapSet<usize>,
    }
    impl SegmentTreeNode for Node2 {}
    let mut st = SegmentTree::with_gen_full(k, |i, j| {
        let mut tree = MultiTreapSet::new();
        for x in i..j {
            tree.insert(next[x]);
        }
        Node2 { tree }
    });

    for _ in 0..q {
        let a = input.read_size() - 1;
        let b = input.read_size();

        out.print_line(st.for_each_mut(a..b, |val: usize, node| val + node.tree.more_or_eq(&b)));
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
