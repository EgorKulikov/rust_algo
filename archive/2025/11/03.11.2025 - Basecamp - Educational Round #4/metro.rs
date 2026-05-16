//{"name":"Metro","group":"Eolymp - Basecamp - Educational Round #4","url":"https://eolymp.com/en/compete/btktopvnh51kpfku7ua54hi0bk/problem/10","interactive":false,"timeLimit":1000,"tests":[{"input":"7 3\n2 1 3\n6 1 4 5\n5 7\n2 1 7 3\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::iter_ext::iter_copied::ItersCopied;
use algo_lib::collections::slice_ext::indices::Indices;
use algo_lib::collections::vec_ext::gen_vec::VecGen;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::eol::EolVec;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let l = input.read_size();
    let lines = input.read_vec::<EolVec<usize>>(l);
    let a_station = input.read_size() - 1;
    let a_line = input.read_size() - 1;
    let b_station = input.read_size() - 1;
    let b_line = input.read_size() - 1;

    let mut pos = n;
    let start = Vec::with_gen(l, |i| {
        let res = pos;
        pos += lines[i].len();
        res
    });
    let mut graph = Graph::new(pos);
    for i in 0..l {
        for j in 1..lines[i].len() {
            graph.add_edge(WeightedEdge::new(start[i] + j - 1, start[i] + j, 2));
            graph.add_edge(WeightedEdge::new(start[i] + j, start[i] + j - 1, 2));
        }
        for j in lines[i].indices() {
            graph.add_edge(WeightedEdge::new(lines[i][j] - 1, start[i] + j, 1));
            graph.add_edge(WeightedEdge::new(start[i] + j, lines[i][j] - 1, 0));
        }
    }
    let a = lines[a_line].copy_position(|x| x == a_station + 1).unwrap() + start[a_line];
    let b = lines[b_line].copy_position(|x| x == b_station + 1).unwrap() + start[b_line];
    out.print_line(graph.distance(a, b).unwrap().0);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
        TaskType::Classic | TaskType::RunTwice => input.is_empty(),
        TaskType::Interactive => true,
    }
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
