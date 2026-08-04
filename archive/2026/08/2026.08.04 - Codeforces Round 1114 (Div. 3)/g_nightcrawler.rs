use algo_lib::collections::multi_set::MultiTreeSet;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::Graph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
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
    let a = input.read_long_vec(n);
    let p = input.read_size_vec(n - 1).dec();

    let graph = Graph::with_parents(&p);
    let mut free = Vec::new();
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> MultiTreeSet<i64> {
        let mut calls = MultiTreeSet::new();
        for e in graph.adj(vert) {
            if e.to() == prev {
                continue;
            }
            let mut call = f.call(e.to(), vert);
            if call.len() > calls.len() {
                swap(&mut call, &mut calls);
            }
            for &k in call.iter() {
                calls.insert(k);
            }
        }
        if calls.is_empty() {
            calls.insert(a[vert]);
        } else {
            let min = calls.pop_first().unwrap();
            free.push(a[vert].min(min));
            calls.insert(a[vert].max(min));
        }
        calls
    });
    let need = dfs.call(0, 0);
    let size = need.len();
    let mut sum = 0;
    for &k in need.iter() {
        sum += k;
    }
    free.sort();
    free.reverse();
    let mut ans = vec![-1; size - 1];
    let mut cur = sum;
    ans.push(cur);
    for i in free {
        cur += i;
        ans.push(cur);
    }
    assert_eq!(ans.len(), n);
    out.print_line(ans);
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
