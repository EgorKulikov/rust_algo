use algo_lib::collections::md_arr::arr2d::{Arr2d, Arr2dRead};
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::numbers::num_traits::bit_ops::BitOps;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let h = input.read_size();
    let w = input.read_size();
    let k = input.read_size();
    let s = input.read_table(h, w);

    let mut graph = Graph::new(h * w, 0);
    let mut start = (h, w);
    let mut finish = (h, w);
    let mut kittens = Vec::new();
    for (i, j) in s.indices() {
        if s[(i, j)] == b'#' {
            continue;
        }
        match s[(i, j)] {
            b'@' => start = (i, j),
            b'G' => finish = (i, j),
            b'F' => kittens.push((i, j)),
            _ => {}
        }
        if i + 1 < h && s[(i + 1, j)] != b'#' {
            graph.add_edge(BiEdge::new(i * w + j, (i + 1) * w + j));
        }
        if j + 1 < w && s[(i, j + 1)] != b'#' {
            graph.add_edge(BiEdge::new(i * w + j, i * w + j + 1));
        }
    }
    assert_eq!(k, kittens.len());
    let d = Vec::with_gen(k, |i| graph.edge_distances(kittens[i].0 * w + kittens[i].1));
    let mut ans = Arr2d::new(k, 1 << k, None);
    for i in 0..k {
        let c = d[i][start.0 * w + start.1];
        if c == u32::MAX {
            out.print_line(-1);
            return;
        }
        ans[(i, 1 << i)] = Some(c);
    }
    for i in usize::iter_all(k) {
        for j in 0..k {
            if let Some(c) = ans[(j, i)] {
                for l in 0..k {
                    if i.is_set(l) {
                        continue;
                    }
                    ans[(l, i.with_bit(l))].minim(c + d[j][kittens[l].0 * w + kittens[l].1]);
                }
            }
        }
    }
    let mut res = None;
    for i in 0..k {
        if let Some(c) = ans[(i, usize::all_bits(k))] {
            let dd = d[i][finish.0 * w + finish.1];
            if dd == u32::MAX {
                out.print_line(-1);
                return;
            }
            res.minim(c + dd);
        }
    }
    out.print_line(res);
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
