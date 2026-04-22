//{"name":"D - Shortest Evacuation Route","group":"AtCoder - AtCoder Weekday Contest 0051 Beta","url":"https://atcoder.jp/contests/awc0051/tasks/awc0051_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 1 10\n1 2 3\n2 4 4\n1 3 5\n3 4 6\n2\n","output":"11\n"},{"input":"6 7 3 5\n1 2 5\n1 3 1\n2 5 2\n3 5 1\n2 4 4\n4 6 3\n5 6 3\n1 3 6\n","output":"20\n"},{"input":"8 10 0 100\n1 2 2\n1 3 10\n2 5 3\n3 4 5\n4 6 2\n5 7 4\n6 7 8\n7 8 1\n3 5 6\n6 8 12\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let t = input.read_long();
    let edges = input.read_vec::<(usize, usize, i64)>(m).dec();
    let g = input.read_size_vec(k).dec();

    let mut v_cost = vec![0; n];
    for i in g {
        v_cost[i] = t;
    }
    let mut graph = Graph::new(2 * n);
    for i in 0..n {
        graph.add_edge(WeightedEdge::new(i, i + n, v_cost[i]));
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
