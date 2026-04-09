//{"name":"D - Optimizing Delivery Routes","group":"AtCoder - AtCoder Weekday Contest 0044 Beta","url":"https://atcoder.jp/contests/awc0044/tasks/awc0044_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 5 1\n1 2 3\n1 3 2\n2 4 5\n3 4 4\n2 3 7\n3\n","output":"6\n"},{"input":"3 1 0\n1 2 5\n","output":"-1\n"},{"input":"8 12 3\n1 2 4\n1 3 7\n2 3 2\n2 4 5\n3 5 3\n4 5 1\n4 6 8\n5 6 6\n5 7 9\n6 8 3\n7 8 2\n3 7 10\n1 6 11\n","output":"19\n"},{"input":"15 20 5\n1 2 10\n1 3 15\n2 4 12\n3 4 8\n3 5 7\n4 6 6\n5 6 9\n5 7 3\n6 8 14\n7 8 5\n7 9 11\n8 10 4\n9 10 2\n9 11 13\n10 12 7\n11 12 6\n11 13 8\n12 14 3\n13 14 10\n14 15 5\n2 5 7 9 19\n","output":"71\n"},{"input":"2 1 1\n1 2 1000000000\n1\n","output":"2000000000\n"},{"input":"4 5 1\n1 2 3\n1 3 2\n2 4 5\n3 4 4\n2 3 7\n3\n","output":"6\n"},{"input":"3 1 0\n1 2 5\n","output":"-1\n"},{"input":"8 12 3\n1 2 4\n1 3 7\n2 3 2\n2 4 5\n3 5 3\n4 5 1\n4 6 8\n5 6 6\n5 7 9\n6 8 3\n7 8 2\n3 7 10\n1 6 11\n","output":"19\n"},{"input":"15 20 5\n1 2 10\n1 3 15\n2 4 12\n3 4 8\n3 5 7\n4 6 6\n5 6 9\n5 7 3\n6 8 14\n7 8 5\n7 9 11\n8 10 4\n9 10 2\n9 11 13\n10 12 7\n11 12 6\n11 13 8\n12 14 3\n13 14 10\n14 15 5\n2 5 7 9 19\n","output":"71\n"},{"input":"2 1 1\n1 2 1000000000\n1\n","output":"2000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let k = input.read_size();
    let mut uvw = input.read_vec::<(usize, usize, i64)>(m).dec();
    let c = input.read_size_vec(k).dec();

    for i in c {
        uvw[i].2 *= 2;
    }
    let mut graph = Graph::new(n);
    for (u, v, w) in uvw {
        graph.add_edge(BiWeightedEdge::new(u, v, w));
    }
    out.print_line(graph.distances_from(0)[n - 1].map(|x| x.0));
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
