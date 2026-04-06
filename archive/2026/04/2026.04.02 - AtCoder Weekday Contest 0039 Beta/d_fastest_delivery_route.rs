//{"name":"D - Fastest Delivery Route","group":"AtCoder - AtCoder Weekday Contest 0039 Beta","url":"https://atcoder.jp/contests/awc0039/tasks/awc0039_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5\n1 2 3\n1 3 5\n2 3 1\n2 4 10\n3 4 2\n","output":"6\n"},{"input":"6 9\n1 2 7\n1 3 9\n1 4 14\n2 3 10\n2 5 15\n3 4 2\n3 5 11\n4 5 9\n5 6 6\n","output":"26\n"},{"input":"10 15\n1 2 1000000000\n1 3 500000000\n2 4 300000000\n3 4 200000000\n3 5 400000000\n4 6 100000000\n5 6 150000000\n5 7 250000000\n6 7 50000000\n6 8 200000000\n7 8 100000000\n7 9 300000000\n8 9 150000000\n8 10 400000000\n9 10 100000000\n","output":"1200000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
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
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n);
    for (u, v, w) in edges {
        graph.add_edge(WeightedEdge::new(u, v, w));
    }

    out.print_line(graph.distances_from(0)[n - 1].unwrap().0);
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
