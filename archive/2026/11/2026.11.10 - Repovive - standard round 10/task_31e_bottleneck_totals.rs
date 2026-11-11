use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::fx_hash_map::FxHashSet;
use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::slice_ext::splits::Split;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::order_by;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::extensions::option::OptionExt;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_size_vec(n);
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_biedges(n, &edges);

    #[derive(Clone)]
    struct Part {
        vert: Vec<usize>,
        delta: usize,
        sizes: Vec<usize>,
    }
    let mut parts = Vec::with_gen(n, |i| Part {
        vert: vec![i],
        delta: 0,
        sizes: vec![1],
    });
    let mut id = Vec::with_gen(n, |i| i);

    let order = order_by(n, |i| a[i]);
    let mut other = vec![Vec::new(); n];
    let mut base = vec![0; n];
    let mut enabled = BitSet::new(n);
    for i in order.copy_rev() {
        let mut base_part = i;
        enabled.set(i);
        let mut seen = FxHashSet::default();
        for e in graph.adj(i) {
            let to = id[e.to()];
            if seen.contains(&to) {
                continue;
            }
            seen.insert(to);
            if !enabled[to] {
                continue;
            }
            if parts[to].vert.len() > parts[base_part].vert.len() {
                other[i].push(base_part);
                base_part = to;
            } else {
                other[i].push(to);
            }
        }
        base[i] = base_part;
        for o in other[i].copy_iter() {
            let (bp, op) = parts.two_mut(base_part, o);
            for v in op.vert.copy_iter() {
                id[v] = base_part;
                bp.vert.push(v);
            }
        }
        let len = parts[base_part].vert.len();
        parts[base_part].sizes.push(len);
    }
    let mut ans = vec![0; n];
    for i in order {
        let b = base[i];
        if a[i] < parts[b].delta {
            out.print_line(-1);
            return;
        }
        let val = a[i] - parts[b].delta;
        let q = parts[b].sizes[Back(0)];
        if val % q != 0 {
            out.print_line(-1);
            return;
        }
        ans[i] = val / q;
        parts[b].sizes.pop();
        for o in other[i].copy_iter() {
            parts[o].delta += ans[i] * (q - parts[o].sizes[Back(0)]) + parts[b].delta;
        }
        parts[b].delta += ans[i] * (q - parts[b].sizes[Back(0)]);
    }
    out.print_line((&ans).take_if(ans.copy_all(|x| x >= 1 && x <= 1_000_000_000)));
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
