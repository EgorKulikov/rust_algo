use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::multi_set::MultiHashSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::collections::vec_ext::sorted::Sorted;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;
use std::mem::swap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let p = input.read_size_vec(n - 1).dec();
    let a = input.read_long_vec(n);

    #[derive(Default)]
    struct Node {
        set: MultiHashSet<i64>,
        len: i64,
    }
    let mut graph = Graph::new(n, n - 1);
    for i in 0..n - 1 {
        graph.add_edge(BiEdge::new(p[i], i + 1));
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> Node {
        let mut res = Node::default();
        for e in graph.adj(vert) {
            if e.to() == prev {
                continue;
            }
            let mut call = f.call(e.to(), vert);
            if call.set.len() > res.set.len() {
                swap(&mut call, &mut res);
            }
            res.len.maxim(call.len);
            for v in call.set.iter().copied() {
                res.set.insert(v);
            }
        }
        res.set.remove(&res.len);
        res.len += a[vert];
        res.set.insert(res.len);
        res
    });
    let mut res = dfs.call(0, 0);
    res.set.insert(0);
    let mut at = res.len;
    let x = res
        .set
        .iter()
        .copied()
        .collect::<Vec<_>>()
        .sorted()
        .reversed();
    let mut w = 0;
    let mut rem = k;
    for i in x {
        if (at - i) * w <= rem {
            rem -= (at - i) * w;
            at = i;
            w += 1;
        } else {
            at -= rem / w;
            out.print_line(at);
            return;
        }
    }
    out.print_line(0);
}

#[allow(unused_variables)]
fn solve2(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    eprint!("\x1B[33m\x1B[03m");
    let mut pre_calc = ();

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
        TestType::RunTwiceSingle => {
            let mode = input.read_str();
            match mode.as_slice() {
                b"first" => solve(&mut input, &mut output, 1, &mut pre_calc),
                b"second" => solve2(&mut input, &mut output, 1, &mut pre_calc),
                _ => unreachable!(),
            }
        }
        TestType::RunTwiceMultiNumber => {
            let mode = input.read_str();
            let t = input.read();
            for i in 1..=t {
                match mode.as_slice() {
                    b"first" => solve(&mut input, &mut output, i, &mut pre_calc),
                    b"second" => solve2(&mut input, &mut output, i, &mut pre_calc),
                    _ => unreachable!(),
                }
            }
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
