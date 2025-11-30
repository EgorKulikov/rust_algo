//{"name":"D - Michishirube","group":"AtCoder - AtCoder Regular Contest 211 (Div. 2)","url":"https://atcoder.jp/contests/arc211/tasks/arc211_d","interactive":false,"timeLimit":2000,"tests":[{"input":"7 8\n1 3\n3 4\n4 5\n1 5\n4 6\n2 6\n2 7\n4 7\n","output":"Yes\n5\n6\n4 1\n5 6\n1 4\n4 2\n4 2\n"},{"input":"10 12\n1 3\n3 4\n4 5\n1 5\n4 6\n2 6\n2 7\n4 7\n4 8\n8 9\n9 10\n8 10\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = input.read_size_pair_vec(m).dec();

    let graph = Graph::with_biedges(n, &edges);
    let mut yellow = vec![n; n];
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            if yellow[e.to()] == n && e.to() != 0 {
                yellow[e.to()] = vert;
                f.call(e.to(), vert);
            }
        }
    });
    dfs.call(0, n);
    let mut blue = vec![n; n];
    let mut seen = 0;
    let mut dfs = RecursiveFunction2::new(|f, vert: usize, prev: usize| {
        seen += 1;
        for e in &graph[vert] {
            if e.to() == prev {
                continue;
            }
            if blue[e.to()] == n && yellow[e.to()] != vert && e.to() != 1 {
                blue[e.to()] = vert;
                f.call(e.to(), vert);
            }
        }
    });
    dfs.call(1, n);
    if seen != n {
        out.print_line(false);
        return;
    }
    out.print_line(true);
    out.print_line(blue[0] + 1);
    out.print_line(yellow[1] + 1);
    for i in 2..n {
        out.print_line((yellow[i] + 1, blue[i] + 1));
    }
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
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
