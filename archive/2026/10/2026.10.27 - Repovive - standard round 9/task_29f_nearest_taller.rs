use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_size_vec(n);

    let mut left_children = vec![Vec::new(); n];
    let mut right_children = vec![Vec::new(); n];
    let mut root = n;
    for i in 0..n {
        if d[i] != 0 {
            if i + d[i] < n {
                left_children[i + d[i]].push(i);
            }
            if i >= d[i] {
                right_children[i - d[i]].push(i);
            }
        } else {
            if root != n {
                out.print_line(-1);
                return;
            }
            root = i;
        }
    }
    if root == n {
        out.print_line(-1);
        return;
    }
    #[derive(Default)]
    struct Node {
        delta: i64,
    }
    impl SegmentTreeNode for Node {
        fn accumulate(&mut self, value: &Self) {
            self.delta += value.delta;
        }
        fn reset_delta(&mut self) {
            self.delta = 0;
        }
    }
    let mut st = SegmentTree::<Node>::new(n);
    for i in 0..n {
        if d[i] != 0 {
            st.update((i + 1).saturating_sub(d[i])..i + d[i], &Node { delta: 1 });
            st.update(i..=i, &Node { delta: -1 });
        }
    }
    let mut ans = vec![0; n];
    let mut next = n - 1;
    let mut pos = vec![0; n];
    ans[root] = n;
    let mut rec = RecursiveFunction2::new(|rec, from: usize, to: usize| -> bool {
        if from == to {
            return true;
        }
        let mut cand = None;
        if to < n {
            let a = &left_children[to];
            let p = a.lower_bound(&from);
            if p < a.len() {
                let vert = a[p];
                if st.point_query(vert).delta == 0 && (cand.is_none() || d[vert] > d[cand.unwrap()])
                {
                    cand = Some(vert);
                }
            }
        }
        if from > 0 {
            let a = &right_children[from - 1];
            let p = a.lower_bound(&to);
            if p > 0 {
                let vert = a[p - 1];
                if st.point_query(vert).delta == 0 && (cand.is_none() || d[vert] > d[cand.unwrap()])
                {
                    cand = Some(vert);
                }
            }
        }
        if let Some(vert) = cand {
            ans[vert] = next;
            pos[next] = vert;
            next -= 1;
            st.update(
                (vert + 1).saturating_sub(d[vert])..vert + d[vert],
                &Node { delta: -1 },
            );
            st.update(vert..=vert, &Node { delta: 1 });
            rec.call(from, vert) && rec.call(vert + 1, to)
        } else {
            false
        }
    });
    if !rec.call(0, root) || !rec.call(root + 1, n) {
        out.print_line(-1);
        return;
    }
    let mut set = BTreeSet::new();
    set.insert(root);
    for i in (1..n).rev() {
        let mut dist = None;
        if let Some(&x) = set.next(&pos[i]) {
            dist.minim(x - pos[i]);
        }
        if let Some(&x) = set.prev(&pos[i]) {
            dist.minim(pos[i] - x);
        }
        if dist != Some(d[pos[i]]) {
            out.print_line(-1);
            return;
        }
        set.insert(pos[i]);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
