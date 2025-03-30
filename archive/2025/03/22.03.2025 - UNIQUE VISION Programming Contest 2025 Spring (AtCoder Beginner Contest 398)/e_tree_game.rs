//{"name":"E - Tree Game","group":"AtCoder - UNIQUE VISION Programming Contest 2025 Spring (AtCoder Beginner Contest 398)","url":"https://atcoder.jp/contests/abc398/tasks/abc398_e","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable3, RecursiveFunction3};
use algo_lib::misc::test_type::TaskType;
use std::collections::BTreeSet;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut col = vec![0; n];
    let mut dfs = RecursiveFunction3::new(|f, vert: usize, prev: usize, c: i32| {
        col[vert] = c;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            f.call(e.to(), vert, -c);
        }
    });
    dfs.call(0, n, 1);
    let mut moves = BTreeSet::new();
    for i in 0..n {
        for j in i + 1..n {
            if col[i] != col[j] {
                moves.insert((i, j));
            }
        }
    }
    for (u, v) in edges {
        moves.remove(&(u, v));
    }
    let first = if moves.len() % 2 == 0 {
        out.print_line("Second");
        false
    } else {
        out.print_line("First");
        true
    };
    out.flush();
    if first {
        let (u, v) = *moves.first().unwrap();
        moves.remove(&(u, v));
        out.print_line((u + 1, v + 1));
        out.flush();
    }
    while !moves.is_empty() {
        let u = input.read_size() - 1;
        let v = input.read_size() - 1;
        moves.remove(&(u, v));
        let (u, v) = *moves.first().unwrap();
        moves.remove(&(u, v));
        out.print_line((u + 1, v + 1));
        out.flush();
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
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
