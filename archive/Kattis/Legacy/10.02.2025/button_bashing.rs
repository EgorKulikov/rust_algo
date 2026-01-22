//{"name":"Button Bashing","group":"Kattis","url":"https://open.kattis.com/problems/buttonbashing","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 50\n-10 10 60\n1 50\n20\n","output":"2 0\n3 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let t = input.read_size();
    let b = input.read_int_vec(n);

    let mut graph = Graph::new(3601);
    for i in 0..=3600 {
        for j in b.copy_iter() {
            let to = (i + j).min(3600).max(0);
            graph.add_edge(Edge::new(i as usize, to as usize));
        }
    }
    let d = graph.edge_distances(0);
    for i in t..=3600 {
        if d[i] != u32::MAX {
            out.print_line((d[i], i - t));
            return;
        }
    }
    unreachable!();
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
