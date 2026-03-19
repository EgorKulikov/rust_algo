//{"name":"I. Ivan Tsarevich in the Magical Grove","group":"Universal Cup - GP of Belarus","url":"https://contest.ucup.ac/contest/3426/problem/17268","interactive":false,"timeLimit":1000,"tests":[{"input":"6 4\n2 0\n1 0\n3 1\n3 0\n3 0\n6 1\n","output":"0 0 -1 1 1 1\n"},{"input":"3 1\n2 0\n3 0\n1 0\n","output":"-2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::btree_ext::BTreeExt;
use algo_lib::collections::default_map::DefaultTreeMap;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let ab = input.read_size_pair_vec(n);

    let mut graph = Graph::new(n);
    for (i, (a, b)) in ab.iter_enumerate() {
        graph.add_edge(BiEdge::with_payload(i, a - 1, 1 - b));
    }
    let mut t = vec![None; n];
    let mut c = vec![2; n];
    let mut one = k;
    let mut other = n - k;
    let mut qty = DefaultTreeMap::new(0);
    for i in 0..n {
        if t[i].is_some() {
            continue;
        }
        let mut w = Vec::new();
        let mut b = Vec::new();
        let mut good = true;
        let mut rec = RecursiveFunction2::new(|rec, vert: usize, col: usize| {
            if c[vert] != 2 {
                if c[vert] != col {
                    good = false;
                }
                return;
            }
            c[vert] = col;
            if col == 0 {
                w.push(vert);
            } else {
                b.push(vert);
            }
            for e in &graph[vert] {
                rec.call(e.to(), col ^ *e.payload());
            }
        });
        rec.call(i, 0);
        if !good {
            out.print_line(-2);
            return;
        }
        if one.min(other) < w.len().min(b.len()) {
            out.print_line(-2);
            return;
        }
        one -= w.len().min(b.len());
        other -= w.len().min(b.len());
        let delta = w.len().abs_diff(b.len());
        if w.len() > b.len() {
            swap(&mut w, &mut b);
        }
        for i in w {
            t[i] = Some((delta, 0));
        }
        for i in b {
            t[i] = Some((delta, 1));
        }
        if delta != 0 {
            qty[delta] += 1;
        }
    }
    let mut at = 0;
    let mut can_double = FxHashSet::default();
    while let Some((&key, &q)) = qty.next(&at) {
        if q >= 3 {
            can_double.insert(key);
            let transfer = (q - 1) / 2;
            qty[key] -= transfer * 2;
            qty[key * 2] += transfer;
        }
        at = key;
    }
    let target = one.min(other);
    let mut d = Vec::new();
    for (delta, q) in qty {
        for _ in 0..q {
            d.push(delta);
        }
    }
    let ltr = Vec::with_gen_prefix(d.len() + 1, |i, v| -> BitSet {
        if i == 0 {
            let mut res = BitSet::new(target + 1);
            res.set(0);
            res
        } else {
            let mut res = v[i - 1].clone();
            res.shift_left_or(d[i - 1]);
            res
        }
    });
    if !ltr[d.len()][target] {
        out.print_line(-2);
        return;
    }
    let rtl = Vec::with_gen_prefix(d.len() + 1, |i, v| -> BitSet {
        let i = d.len() - i;
        if i == d.len() {
            let mut res = BitSet::new(target + 1);
            res.set(target);
            res
        } else {
            let mut res = v[d.len() - i - 1].clone();
            res.shift_right_or(d[i]);
            res
        }
    })
    .reversed();
    let mut can_skip = BitSet::new(n + 1);
    let mut can_take = BitSet::new(n + 1);
    for i in d.indices() {
        let mut cur = ltr[i].clone();
        cur &= &rtl[i + 1];
        if cur.count_ones() != 0 {
            can_skip.set(d[i]);
        }
        cur = ltr[i].clone();
        cur <<= d[i];
        cur &= &rtl[i + 1];
        if cur.count_ones() != 0 {
            can_take.set(d[i]);
        }
    }
    for i in (1..=n).rev() {
        if 2 * i <= n && can_double.contains(&i) {
            if can_skip[2 * i] {
                can_skip.set(i);
            }
            if can_take[2 * i] {
                can_take.set(i);
            }
        }
    }
    let mut ans = vec![0; n];
    let factor = if one < other { 1 } else { -1 };
    for i in 0..n {
        let (delta, col) = t[i].unwrap();
        if delta == 0 {
            continue;
        }
        assert!(can_skip[delta] || can_take[delta]);
        if !can_skip[delta] {
            ans[i] = factor * (col * 2 - 1);
        } else if !can_take[delta] {
            ans[i] = -factor * (col * 2 - 1);
        }
    }
    out.print_line(ans);
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
