//{"name":"D - Network Installation","group":"AtCoder - AtCoder Weekday Contest 0029 Beta","url":"https://atcoder.jp/contests/awc0029/tasks/awc0029_d","interactive":false,"timeLimit":2000,"tests":[{"input":"5 6 5\n1 2 3\n1 3 7\n2 5 10\n3 4 6\n4 5 8\n1 5 2\n","output":"3\n"},{"input":"4 4 10\n1 2 15\n1 3 5\n2 3 8\n3 4 12\n","output":"-1\n"},{"input":"8 10 4\n1 2 3\n1 3 10\n1 4 7\n2 5 6\n3 5 5\n4 6 8\n5 7 2\n5 8 9\n6 7 12\n7 8 6\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_int();
    let uvw = input.read_vec::<(usize, usize, i32)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in uvw {
        if w >= k {
            graph.add_edge(BiEdge::new(u, v));
        }
    }
    let d = graph.edge_distances(0);
    if d[n - 1] == u32::MAX {
        out.print_line(-1);
    } else {
        out.print_line(d[n - 1]);
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
