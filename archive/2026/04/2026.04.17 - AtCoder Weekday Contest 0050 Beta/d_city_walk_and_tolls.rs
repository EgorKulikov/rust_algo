//{"name":"D - City Walk and Tolls","group":"AtCoder - AtCoder Weekday Contest 0050 Beta","url":"https://atcoder.jp/contests/awc0050/tasks/awc0050_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 1\n1 2 2\n2 3 2\n3 4 2\n1 4 10\n3 5\n","output":"10\n"},{"input":"6 7 3\n1 2 3\n1 3 5\n2 4 4\n3 4 1\n4 5 2\n5 6 3\n2 5 10\n2 6\n4 3\n5 1\n","output":"15\n"},{"input":"8 10 4\n1 2 5\n1 3 3\n2 4 2\n3 5 4\n4 6 6\n5 6 2\n6 7 3\n7 8 4\n5 7 8\n1 4 10\n1 2\n5 3\n6 1\n8 4\n","output":"26\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let k = input.read_size();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();
    let lc = input.read_vec::<(usize, i64)>(k);

    let mut cost = vec![0; n];
    for (l, c) in lc {
        cost[l - 1] = c;
    }
    let mut graph = Graph::new(2 * n);
    for i in 0..n {
        graph.add_edge(WeightedEdge::new(i, i + n, cost[i]));
    }
    for (u, v, w) in edges {
        graph.add_edge(WeightedEdge::new(u + n, v, w));
        graph.add_edge(WeightedEdge::new(v + n, u, w));
    }
    out.print_line(graph.distance(0, 2 * n - 1).unwrap().0);
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
    #[cfg(debug_assertions)]
    eprintln!("Library code is available at https://github.com/EgorKulikov/rust_algo");
    let input = algo_lib::io::input::Input::stdin();
    let output = algo_lib::io::output::Output::stdout();
    run(input, output);
}
