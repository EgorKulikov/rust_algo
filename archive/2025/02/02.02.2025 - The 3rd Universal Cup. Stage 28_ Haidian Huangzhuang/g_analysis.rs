//{"name":"G. Analysis","group":"Universal Cup - The 3rd Universal Cup. Stage 28: Haidian Huangzhuang","url":"https://contest.ucup.ac/contest/1903/problem/9696","interactive":false,"timeLimit":1000,"tests":[{"input":"5 100 1000\n1 2\n2 3\n3 4\n4 5\n","output":"0\n"},{"input":"5 100 200\n1 2\n1 3\n2 4\n2 5\n","output":"100\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::slice_ext::backward::Back;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::BiEdgeAlgos;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long();
    let b = input.read_long();
    let edges = input.read_size_pair_vec(n - 1).dec();

    let graph = Graph::with_biedges(n, &edges);
    let diam = graph.diameter();
    if b > a * diam as i64 {
        out.print_line(a * (n - 1 - diam) as i64);
        return;
    }
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| -> (i64, i64) {
        let mut calls = Vec::new();
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            let call = f.call(e.to(), vert);
            calls.push(call);
        }
        calls.sort_unstable_by_key(|&(x, y)| x - y);
        let sum_y: i64 = calls.iter().map(|&(_, y)| y).sum();
        let mut y = sum_y;
        let mut cur_y = sum_y;
        for i in (2..=calls.len()).step_by(2) {
            cur_y -= calls[i - 2].1 + calls[i - 1].1;
            cur_y += calls[i - 2].0 + calls[i - 1].0 + b;
            y.minim(cur_y);
        }
        if calls.len() % 2 == 1 {
            cur_y -= calls[Back(0)].1;
            cur_y += calls[Back(0)].0 + b;
            y.minim(cur_y);
        }
        let mut x = -a + sum_y;
        if !calls.is_empty() {
            x += calls[0].0 - calls[0].1;
        }
        let mut cur_x = x;
        for i in (3..=calls.len()).step_by(2) {
            cur_x -= calls[i - 2].1 + calls[i - 1].1;
            cur_x += calls[i - 2].0 + calls[i - 1].0 + b;
            x.minim(cur_x);
        }
        if calls.len() % 2 == 0 && !calls.is_empty() {
            cur_x -= calls[Back(0)].1;
            cur_x += calls[Back(0)].0 + b;
            x.minim(cur_x);
        }
        (x, y)
    });
    let (_, ans) = dfs.call(0, n);
    out.print_line(ans + a * (n as i64 - 1) - b);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

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

//START MAIN
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
//END MAIN
