//{"name":"D - Teleport Maze","group":"AtCoder - AtCoder Beginner Contest 436","url":"https://atcoder.jp/contests/abc436/tasks/abc436_d","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n..a.\n####\nba#b\n","output":"5\n"},{"input":"3 4\n..a.\n####\nb.#b\n","output":"-1\n"},{"input":"4 4\nxxxx\nxxxx\nxxxx\nxxxx\n","output":"1\n"},{"input":"7 11\nu..#y..#...\nk..#.z.#.k.\niju#...#x..\n###########\n..x#.t.#..n\nabc#y..#...\n..z#..t#.y.\n","output":"12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

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
    let h = input.read_size();
    let w = input.read_size();
    let s = input.read_char_table(h, w);

    let mut graph = Graph::new(h * w + 26);
    for (r, c) in s.indices() {
        if s[(r, c)] == b'#' {
            continue;
        }
        if s[(r, c)].is_ascii_alphabetic() {
            graph.add_edge(WeightedEdge::new(
                r * w + c,
                h * w + s[(r, c)] as usize - b'a' as usize,
                1,
            ));
            graph.add_edge(WeightedEdge::new(
                h * w + s[(r, c)] as usize - b'a' as usize,
                r * w + c,
                0,
            ));
        }
        for (nr, nc) in D4::iter(r, c, h, w) {
            if s[(nr, nc)] != b'#' {
                graph.add_edge(WeightedEdge::new(r * w + c, nr * w + nc, 1));
            }
        }
    }
    out.print_line(graph.zero_one_distances_from(0)[h * w - 1].map(|x| x.0));
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
