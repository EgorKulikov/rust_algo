//{"name":"Grid","group":"Kattis","url":"https://open.kattis.com/problems/grid","interactive":false,"timeLimit":1000,"tests":[{"input":"2 2\n11\n11\n","output":"2\n"},{"input":"2 2\n22\n22\n","output":"-1\n"},{"input":"5 4\n2120\n1203\n3113\n1120\n1110\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Grid"}}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
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
    let m = input.read_size();
    let table = input.read_char_table(n, m);

    let mut graph = Graph::new(n * m);
    for i in 0..n {
        for j in 0..m {
            let steps = table[i][j] as usize - '0' as usize;
            if i >= steps {
                graph.add_edge(Edge::new(i * m + j, (i - steps) * m + j));
            }
            if i + steps < n {
                graph.add_edge(Edge::new(i * m + j, (i + steps) * m + j));
            }
            if j >= steps {
                graph.add_edge(Edge::new(i * m + j, i * m + j - steps));
            }
            if j + steps < m {
                graph.add_edge(Edge::new(i * m + j, i * m + j + steps));
            }
        }
    }
    let ans = graph.edge_distances(0)[n * m - 1];
    if ans == u32::MAX {
        out.print_line(-1);
    } else {
        out.print_line(ans);
    }
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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);

    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);

    run(input, output);
}
//END MAIN
