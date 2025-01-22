//{"name":"Simultaneous Robots","group":"CodeChef - START170A","url":"https://www.codechef.com/START170A/problems/ROBO2","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n2 2\n.#\n#.\n5 5\n.....\n.###.\n..##.\n#.#..\n#....\n","output":"-1\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::graph::edge_distances::EdgeAlgos;
use algo_lib::graph::edges::edge::Edge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_char_table(n, m);

    if a[(0, 1)] == b'#'
        || a[(1, 0)] == b'#'
        || a[(n - 1, m - 2)] == b'#'
        || a[(n - 2, m - 1)] == b'#'
    {
        out.print_line(-1);
        return;
    }

    let mut graph = Graph::new(n * m);
    for i in 0..n {
        for j in 0..m {
            if a[(i, j)] == b'#' {
                continue;
            }
            for (r, c) in D4::iter(i, j, n, m) {
                if a[(r, c)] == b'.' {
                    graph.add_edge(Edge::new(i * m + j, r * m + c));
                }
            }
        }
    }
    let d0 = graph.edge_distances(0);
    let d1 = graph.edge_distances(n * m - 1);
    if d0[n * m - 1] == u32::MAX {
        out.print_line(-1);
        return;
    }
    let mut qty = vec![0; d0[n * m - 1] as usize + 1];
    for i in 0..n * m {
        if d0[i].saturating_add(d1[i]) == d0[n * m - 1] {
            qty[d0[i] as usize] += 1;
        }
    }
    for i in 1..d1[0] as usize {
        if qty[i] == 1 {
            out.print_line(d1[0] + 2);
            return;
        }
    }
    out.print_line(d1[0]);
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
