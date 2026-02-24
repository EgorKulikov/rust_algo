//{"name":"D - Escape from the Maze","group":"AtCoder - AtCoder Weekday Contest 0012 Beta","url":"https://atcoder.jp/contests/awc0012/tasks/awc0012_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 5\n...#.\n.###.\n...#.\n","output":"1\n"},{"input":"3 3\n...\n...\n...\n","output":"0\n"},{"input":"5 7\n..#.#.#\n.#...#.\n##.#.##\n.#...#.\n..#.#..\n","output":"2\n"},{"input":"10 10\n..........\n.########.\n..........\n.########.\n..........\n.########.\n..........\n.########.\n..........\n..........\n","output":"0\n"},{"input":"1 1\n.\n","output":"0\n"},{"input":"3 5\n...#.\n.###.\n...#.\n","output":"1\n"},{"input":"3 3\n...\n...\n...\n","output":"0\n"},{"input":"5 7\n.#.#.#.\n.#.#.#.\n.#.#.#.\n.#.#.#.\n.......\n","output":"0\n"},{"input":"10 10\n..........\n.########.\n..........\n########.#\n..........\n.########.\n..........\n########.#\n..........\n..........\n","output":"0\n"},{"input":"1 1\n.\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::BoolOutput;
use algo_lib::io::output::Output;
use algo_lib::misc::dirs::D4;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let s = input.read_char_table(n, m);

    let mut graph = Graph::new(n * m);
    for (r, c) in s.indices() {
        for (nr, nc) in D4::iter(r, c, n, m) {
            graph.add_edge(WeightedEdge::new(
                r * m + c,
                nr * m + nc,
                if s[(nr, nc)] == b'#' { 1 } else { 0 },
            ));
        }
    }
    out.print_line(graph.zero_one_distances_from(0)[n * m - 1].unwrap().0);
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
