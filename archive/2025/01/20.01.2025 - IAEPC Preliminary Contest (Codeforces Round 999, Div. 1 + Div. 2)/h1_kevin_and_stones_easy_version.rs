//{"name":"H1. Kevin and Stones (Easy Version)","group":"Codeforces - IAEPC Preliminary Contest (Codeforces Round 999, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2061/problem/H1","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n2 1\n10\n01\n1 2\n11 11\n11011001010\n01101011100\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n8 9\n9 10\n10 11\n11 1\n3 2\n110\n101\n1 2\n2 3\n3 2\n111\n111\n1 2\n2 3\n","output":"Yes\nYes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::flow_edge::FlowEdge;
use algo_lib::graph::max_flow::MaxFlow;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::test_type::TaskType;
use std::mem::swap;

use algo_lib::misc::test_type::TestType;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut s = input.read_str();
    let mut t = input.read_str();
    let edges = input.read_size_pair_vec(m).dec();

    if s == t {
        out.print_line(true);
        return;
    }
    for _ in 0..2 {
        let mut flow_graph = Graph::new(2 * n + 2);
        let source = 2 * n;
        let sink = 2 * n + 1;
        for (u, v) in edges.copy_iter() {
            flow_graph.add_edge(FlowEdge::new(u, v + n, 1));
            flow_graph.add_edge(FlowEdge::new(v, u + n, 1));
        }
        for i in 0..n {
            if s[i] == b'1' {
                flow_graph.add_edge(FlowEdge::new(source, i, 1));
            }
            flow_graph.add_edge(FlowEdge::new(i + n, sink, 1));
        }
        let ones = s.copy_count(b'1');
        if flow_graph.max_flow(source, sink) != ones {
            out.print_line(false);
            return;
        }
        swap(&mut s, &mut t);
    }

    let graph = Graph::with_biedges(n, &edges);
    let mut color = vec![0; n];
    let mut can_odd = true;
    let mut can_even = true;
    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        let mut two_color = true;
        let mut white_s = 0;
        let mut black_s = 0;
        let mut white_t = 0;
        let mut black_t = 0;
        let mut dfs = RecursiveFunction2::new(|f, vert: usize, col: i32| {
            if color[vert] != 0 {
                if color[vert] != col {
                    two_color = false;
                }
                return;
            }
            color[vert] = col;
            if s[vert] == b'1' {
                if col == 1 {
                    white_s += 1;
                } else {
                    black_s += 1;
                }
            }
            if t[vert] == b'1' {
                if col == 1 {
                    white_t += 1;
                } else {
                    black_t += 1;
                }
            }
            for e in &graph[vert] {
                f.call(e.to(), -col);
            }
        });
        dfs.call(i, 1);
        if white_s + black_s != white_t + black_t {
            out.print_line(false);
            return;
        }
        if two_color {
            if (white_s, black_s) != (white_t, black_t) {
                can_even = false;
            }
            if (white_s, black_s) != (black_t, white_t) {
                can_odd = false;
            }
        }
    }
    out.print_line(can_even || can_odd);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
