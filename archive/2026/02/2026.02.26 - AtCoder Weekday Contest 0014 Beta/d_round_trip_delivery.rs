//{"name":"D - Round-Trip Delivery","group":"AtCoder - AtCoder Weekday Contest 0014 Beta","url":"https://atcoder.jp/contests/awc0014/tasks/awc0014_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 3\n1 2 5\n2 3 3\n3 4 2\n1 4 10\n","output":"16\n"},{"input":"5 3 5\n1 2 100\n3 4 50\n4 5 30\n","output":"-1\n"},{"input":"6 8 4\n1 2 10\n1 3 15\n2 3 5\n2 4 20\n3 4 8\n3 5 12\n4 6 6\n5 6 3\n","output":"46\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
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
    let t = input.read_size() - 1;
    let abc = input.read_vec::<(usize, usize, i64)>(m).dec();

    let mut graph = Graph::new(n);
    for (a, b, c) in abc {
        graph.add_edge(BiWeightedEdge::new(a, b, c));
    }
    out.print_line(graph.distance(0, t).map(|x| x.0 * 2));
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
