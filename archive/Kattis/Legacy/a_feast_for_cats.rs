//{"name":"A Feast For Cats","group":"Kattis","url":"https://open.kattis.com/problems/cats","interactive":false,"timeLimit":6000,"tests":[{"input":"1\n20 5\n0 1 4\n0 2 3\n0 3 10\n0 4 15\n1 2 7\n1 3 3\n1 4 5\n2 3 4\n2 4 3\n3 4 8\n","output":"yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::graph::edges::bi_weighted_edge::BiWeightedEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use algo_lib::graph::minimal_spanning_tree::MinimalSpanningTree;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::{BoolOutput, Output};
use algo_lib::misc::extensions::do_with::DoWith;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_int();
    let c = input.read_size();
    let edges: Vec<(usize, usize, i32)> = input.read_vec(c * (c - 1) / 2);

    let graph = Graph::new(c).do_with(|g| {
        for (u, v, w) in edges {
            g.add_edge(BiWeightedEdge::new(u, v, w));
        }
    });
    let tree = graph.minimal_spanning_tree();
    let mut total = c as i32;
    for i in 0..c {
        for e in &tree[i] {
            if e.to() < i {
                total += e.weight();
            }
        }
    }
    out.print_line(total <= m);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();
    output.set_bool_output(BoolOutput::Custom("yes", "no"));

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
