//{"name":"Blackjack","group":"Eolymp - Basecamp - Weekend Practice #17","url":"https://eolymp.com/en/compete/68ise1j0d17fp04bqe407psjqg/problem/5","interactive":false,"timeLimit":5000,"tests":[{"input":"5\n3 -6 5 -2 4\n1 3 2 10 1\n2 4 3 10 5\n","output":"22\n"},{"input":"4\n1 5 -3 2\n2 1 3 2\n4 6 2 5\n","output":"14\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::payload::ValueDeltaPayload;
use algo_lib::collections::segment_tree::{SegmentTree, SegmentTreeNode};
use algo_lib::collections::treap::treap::Tree;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::misc::value_delta::ValueTrait;
use algo_lib::numbers::num_traits::algebra::Zero;
use algo_lib::numbers::num_utils::PartialSums;
use algo_lib::numbers::rational::Rational;
use std::collections::BTreeSet;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let k = input.read_size_vec(n);
    let x = input.read_long_vec(n);

    let s = a.partial_sums();
    let m = Vec::with_gen_back(n + 1, |i, v| if i == n { s[i] } else { s[i].max(v[i + 1]) });
    let mut ans = None;
    for i in 0..n {
        if i + k[i] <= n {
            ans.maxim(k[i] as i64 * x[i] + m[i + k[i].max(1)] - s[i]);
        }
    }
    if x.copy_count(x[0]) == n {
        let a = Vec::with_gen(n, |i| a[i] + x[0]);
        let s = a.partial_sums();
        #[derive(Default, Clone)]
        struct Node {
            val: i64,
        }
        impl SegmentTreeNode for Node {
            fn update(&mut self, left_val: &Self, right_val: &Self) {
                self.val = left_val.val.max(right_val.val);
            }
        }
        let mut st = SegmentTree::with_gen(n + 1, |i| Node { val: s[i] });
        for i in 0..n {
            if k[i] > 0 {
                ans.maxim(st.query(i + 1..=(i + k[i]).min(n)).val - s[i]);
            }
        }
        out.print_line(ans);
        return;
    }
    let mut sp = vec![n];
    let mut last = n;
    for i in (0..n).rev() {
        if s[i] > s[last] {
            last = i;
            sp.push(i);
        }
    }
    sp.reverse();
    let mut delta = vec![Rational::zero(); n];
    let mut rem = BTreeSet::new();
    for i in sp.copy_iter() {
        rem.insert(i);
    }
    let mut next_remove = BTreeSet::new();
    for i in 0..sp.len() - 1 {
        let d = s[sp[i]] - s[sp[i + 1]];
        assert!(d > 0);
        delta[sp[i]] = Rational::new(d as i128, (sp[i + 1] - sp[i]) as i128);
        next_remove.insert((delta[sp[i]], sp[i]));
    }
    struct Node;
    impl ValueTrait for Node {
        type V = (usize, Rational<i128>);

        fn join(v1: Self::V, v2: Self::V) -> Self::V {
            if v1.1 > v2.1 {
                v1
            } else {
                v2
            }
        }
    }
    let mut tree = Tree::new();
    let order = (0..n).collect::<Vec<_>>().sorted_by_key(|&i| x[i]);
    for i in order {
        while let Some(&(t, idx)) = next_remove.first() {
            if t > Rational::new_int(x[i] as i128) {
                break;
            }
            next_remove.remove(&(t, idx));
            rem.remove(&idx);
            tree.insert(ValueDeltaPayload::<Node>::new((idx, t)));
            if let Some(&prev) = rem.prev(&idx) {
                next_remove.remove(&(delta[prev], prev));
                let next = *rem.next(&idx).unwrap();
                let d = s[prev] - s[next];
                assert!(d > 0);
                delta[prev] = Rational::new(d as i128, (next - prev) as i128);
                assert!(delta[prev] >= t);
                next_remove.insert((delta[prev], prev));
            }
        }
        let mut pos = *rem.next(&i).unwrap();
        if pos - i >= k[i] {
            if let Some(p) = tree
                .range(
                    &(i, Rational::new_int((x[i] + 1) as i128))
                        ..&(i + k[i], Rational::new_int((x[i] + 1) as i128)),
                )
                .payload()
            {
                pos = p.v.0;
            } else {
                continue;
            }
        }
        ans.maxim((pos - i) as i64 * x[i] + s[pos] - s[i]);
    }
    out.print_line(ans.unwrap());
}

pub static TEST_TYPE: TestType = TestType::Single;
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
