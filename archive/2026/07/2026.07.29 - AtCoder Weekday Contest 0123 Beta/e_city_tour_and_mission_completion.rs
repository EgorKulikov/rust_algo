use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();
    let t = input.read_long();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();
    let mut s = Vec::new();
    let mut p = Vec::new();
    let mut all = Vec::new();
    for _ in 0..k {
        let len = input.read_size();
        let cur = input.read_size_vec(len).dec();
        all.extend_from_slice(&cur);
        s.push(cur);
        p.push(input.read_long());
    }

    all.sort();
    all.dedup();
    for s in s.iter_mut() {
        for x in s {
            *x = all.lower_bound(&x);
        }
    }
    let mut graph = Graph::new(n, m);
    for (u, v, w) in edges {
        graph.add_edge(WeightedEdge::new(u, v, w));
    }
    let mut res = Arr2d::new(1 << all.len(), all.len(), None);
    let d = graph.distances_from(0);
    for i in all.indices() {
        if let Some((w, ..)) = d[all[i]] {
            res[(1 << i, i)] = Some(w);
        }
    }
    let d = Vec::with_gen(all.len(), |i| graph.distances_from(all[i]));
    for i in usize::iter_all(all.len()) {
        for j in all.indices() {
            if let Some(w) = res[(i, j)] {
                for k in all.indices() {
                    if i.is_set(k) {
                        continue;
                    }
                    if let Some((w2, ..)) = d[j][all[k]] {
                        res[(i | (1 << k), k)] = Some(res[(i | (1 << k), k)].unwrap_or(i64::MAX).min(w + w2));
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for i in usize::iter_all(all.len()) {
        let mut found = false;
        for j in all.indices() {
            if let Some(w) = res[(i, j)] {
                if w <= t {
                    found = true;
                    break;
                }
            }
        }
        if found {
            let mut cur = 0;
            for j in 0..k {
                let mut good = true;
                for x in s[j].copy_iter() {
                    if !i.is_set(x) {
                        good = false;
                        break;
                    }
                }
                if good {
                    cur += p[j];
                }
            }
            ans.maxim(cur);
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
