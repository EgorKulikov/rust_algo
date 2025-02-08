//{"name":"D. Simple Tree","group":"Codeforces - AlgoChief Sprint Round 3","url":"https://codeforces.com/gym/105705/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n6\n3 7 2 5 8 1\n1 2\n2 3\n3 4\n4 5\n5 6\n5\n2 2 2 2 2\n5 3\n4 3\n2 5\n1 2\n","output":"1 1 1 1 1 1\n-1 -1 -1 -1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::vec_ext::inc_dec::IncDec;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let val = input.read_int_vec(n);
    let edges = input.read_size_pair_vec(n - 1).dec();

    let mut graph = Graph::with_biedges(n + 1, &edges);
    for (u, v) in edges {
        if val[u] != val[v] {
            graph.add_edge(BiEdge::new(n, v));
            graph.add_edge(BiEdge::new(n, u));
        }
    }
    let d = graph.edge_distances(n);
    out.print_line_iter(
        d.iter_map(|x| if x == u32::MAX { -1 } else { x as i32 })
            .take(n),
    );
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
